//! The profiles' `sh:sparql` constraints, run as written.
//!
//! cimoxide checks these constraints with hand-written rules ([`crate::sparql`]). This
//! module runs the profiles' own queries instead, with oxigraph through `cimsparql`: as a
//! reference for the hand-written rules, and for constraints nobody has written by hand.
//!
//! A model's files go into one store as their merge, where a triple two files both state is
//! one triple (an end's `rdf:type`, restated by every profile that adds to it, would count
//! twice in a union of the files' graphs), and an NC file also into a named graph of its own
//! ([`graph_name`]):
//!
//! - **CGMES** constraints run on the merge, as ENTSO-E's reference validation runs the
//!   Complex shapes they all sit in, gated by their file's tag
//!   ([`cimschema::shacl::cgmes_manifest::sparql_tags`]): its profile in play, `!NS` when the
//!   model is not solved, `!S` when it is, `COMMON` with common checks on.
//! - **NC** constraints follow NC's manifests: a file a profile's manifest imports runs on
//!   each file of that profile, one only the combined manifest imports on the merge.
//!
//! Each query runs once, for all its focus nodes: `$this` becomes a variable joined with the
//! shape's targets, matched literally as the profiles are written (a `sh:targetClass` takes
//! that class's instances, not its subclasses'), and `$PATH` becomes the shape's path.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use cimmodel::CimDataset;
use cimschema::family::{self, Family};
use cimschema::profile_file::{Pool, Role};
use cimschema::shacl::model::{SparqlConstraint, SparqlTarget};
use cimsparql::{CimStore, QueryResults, Term};

use crate::{Config, Violation};

/// One constraint, ready to run.
#[derive(Debug, Clone)]
pub struct Rule {
    pub constraint: SparqlConstraint,
    pub family: &'static Family,
    /// Where it runs: the tags of its file (CGMES) or the profiles whose manifests import
    /// it (NC, `ALL` for the combined manifest).
    pub tags: Vec<String>,
    /// The query with `$this` and `$PATH` resolved and the targets joined in.
    pub query: String,
    /// Its constraint file's stem.
    pub file: String,
}

/// The constraints of a set of profile files.
#[derive(Debug, Default)]
pub struct SparqlRules {
    pub rules: Vec<Rule>,
    /// Constraints that could not be prepared, with why.
    pub skipped: Vec<(String, String)>,
}

impl SparqlRules {
    /// Every constraint in the files a [`Pool`] found: each family's constraint files, and
    /// NC's manifests for where its constraints run.
    pub fn from_pool(pool: &Pool) -> Result<SparqlRules, String> {
        let mut out = SparqlRules::default();
        for family in family::FAMILIES {
            let nc_tags = if family.shacl_manifest.is_none() {
                Some(
                    cimschema::shacl::resolve::profile_files_of(
                        &pool.files(family, Role::Manifest),
                    )
                    .map_err(|e| format!("{} manifests: {e}", family.id))?,
                )
            } else {
                None
            };
            for path in pool.files(family, Role::Shapes) {
                out.add_file(family, &path, nc_tags.as_ref())?;
            }
        }
        Ok(out)
    }

    fn add_file(
        &mut self,
        family: &'static Family,
        path: &std::path::Path,
        nc_tags: Option<&HashMap<String, Vec<String>>>,
    ) -> Result<(), String> {
        let file = cimschema::shacl::ttl_import::import_ttl_file(path)
            .map_err(|e| format!("{}: {e}", path.display()))?;
        if file.sparql.is_empty() {
            return Ok(());
        }
        let tags: Vec<String> = match nc_tags {
            Some(map) => map.get(&file.file_name).cloned().unwrap_or_default(),
            None => cimschema::shacl::cgmes_manifest::sparql_tags(&file.file_name)
                .into_iter()
                .map(String::from)
                .collect(),
        };
        for constraint in file.sparql {
            if constraint.deactivated {
                continue;
            }
            if tags.is_empty() {
                self.skipped.push((
                    constraint.rule_id.clone(),
                    format!("{} is in no manifest", file.file_name),
                ));
                continue;
            }
            match prepare(&constraint, &file.prefixes) {
                Ok(query) => self.rules.push(Rule {
                    constraint,
                    family,
                    tags: tags.clone(),
                    query,
                    file: file.file_name.clone(),
                }),
                Err(e) => self.skipped.push((constraint.rule_id.clone(), e)),
            }
        }
        Ok(())
    }
}

/// What a run found.
#[derive(Debug, Default)]
pub struct Report {
    /// Each finding, with the index of the file it was found in, or `None` for one found
    /// on the merge, as [`crate::validate_files_traced`] gives them.
    pub violations: Vec<(Option<usize>, Violation)>,
    /// Rules whose query failed to parse or to run, with the error.
    pub failed: Vec<(String, String)>,
    /// How many rules ran.
    pub ran: usize,
}

/// The name of a file's graph: its model's IRI (`md:FullModel`, or NC's `dcat:Dataset`),
/// else one made from its position.
pub fn graph_name(ds: &CimDataset, index: usize) -> String {
    ["FullModel", "nc:Dataset"]
        .iter()
        .find_map(|t| ds.by_type.get(*t).and_then(|ids| ids.first()))
        .map(|id| cimsparql::iri::mrid_to_iri(id))
        .unwrap_or_else(|| format!("urn:cimoxide:file:{index}"))
}

/// Run `rules` on a model's files, as `cfg` puts it: each file's own rules on its graph,
/// the rest on the merge.
pub fn validate_files(per_file: &[CimDataset], cfg: &Config, rules: &SparqlRules) -> Report {
    let mut names: Vec<String> = per_file
        .iter()
        .enumerate()
        .map(|(i, ds)| graph_name(ds, i))
        .collect();
    // Two files declaring one model would share a graph; they get one each.
    let mut seen = HashSet::new();
    for (i, name) in names.iter_mut().enumerate() {
        if !seen.insert(name.clone()) {
            *name = format!("urn:cimoxide:file:{i}");
        }
    }
    let mut report = Report::default();
    let file_profiles: Vec<Vec<String>> = per_file
        .iter()
        .map(|ds| {
            let mut p = crate::detect::detect_config(ds).profiles;
            p.extend(crate::detect::detect_nc_profiles(ds));
            p
        })
        .collect();
    let nc_codes = crate::nc_profile_index().1;
    let is_nc = |profiles: &[String]| profiles.iter().any(|p| nc_codes.contains(&p.as_str()));
    // Every file goes into the merge; an NC file into a graph of its own too, for the
    // rules its profiles' manifests run on each file. CGMES's all run on the merge.
    let graphs: Vec<(Option<&str>, &CimDataset)> = names
        .iter()
        .zip(per_file)
        .zip(&file_profiles)
        .map(|((name, ds), profiles)| (is_nc(profiles).then_some(name.as_str()), ds))
        .collect();
    let store = match CimStore::from_datasets(&graphs) {
        Ok(s) => s,
        Err(e) => {
            report
                .failed
                .push(("(store)".into(), format!("loading the files: {e}")));
            return report;
        }
    };

    let has = |p: &str| cfg.profiles.is_empty() || cfg.profiles.iter().any(|x| x == p);
    let nc_in_play = file_profiles.iter().any(|p| is_nc(p)) || is_nc(&cfg.profiles);

    for rule in &rules.rules {
        if rule.family.shacl_manifest.is_some() {
            if rule.tags.iter().any(|t| cgmes_applies(t, cfg, &has)) {
                run(&store, rule, &[], None, per_file, &mut report);
            }
            continue;
        }
        for tag in &rule.tags {
            if tag == crate::MERGED_PROFILE {
                if nc_in_play {
                    run(&store, rule, &[], None, per_file, &mut report);
                }
                continue;
            }
            if !has(tag) {
                continue;
            }
            for (i, profiles) in file_profiles.iter().enumerate() {
                if profiles.iter().any(|p| p == tag) {
                    run(&store, rule, &[&names[i]], Some(i), per_file, &mut report);
                }
            }
        }
    }

    // One finding per rule and object, as the hand-written rules report.
    let mut once = HashSet::new();
    report
        .violations
        .retain(|(_, v)| once.insert((v.rule_id.clone(), v.object_id.clone())));
    if !cfg.silenced_rules.is_empty() {
        let silenced: HashSet<&str> = cfg.silenced_rules.iter().map(String::as_str).collect();
        report
            .violations
            .retain(|(_, v)| !silenced.contains(v.rule_id.as_str()));
    }
    report
}

/// Whether a CGMES tag applies under `cfg`.
fn cgmes_applies(tag: &str, cfg: &Config, has: &dyn Fn(&str) -> bool) -> bool {
    match tag {
        "COMMON" => cfg.common,
        "COMMON!S" => cfg.common && cfg.solved,
        "HDR" => true,
        _ => {
            if let Some(p) = tag.strip_suffix("!NS") {
                has(p) && cfg.not_solved
            } else if let Some(p) = tag.strip_suffix("!S") {
                has(p) && cfg.solved
            } else {
                has(tag.strip_prefix("X:").unwrap_or(tag))
            }
        }
    }
}

fn run(
    store: &CimStore,
    rule: &Rule,
    graphs: &[&str],
    file: Option<usize>,
    per_file: &[CimDataset],
    report: &mut Report,
) {
    report.ran += 1;
    let solutions = match store.query_in(&rule.query, graphs) {
        Ok(QueryResults::Solutions(s)) => s,
        Ok(_) => {
            report
                .failed
                .push((rule.constraint.rule_id.clone(), "not a SELECT query".into()));
            return;
        }
        Err(e) => {
            report
                .failed
                .push((rule.constraint.rule_id.clone(), e.to_string()));
            return;
        }
    };
    // The files a focus node is looked up in: the one queried, or all.
    let datasets: Vec<&CimDataset> = match file {
        Some(i) => vec![&per_file[i]],
        None => per_file.iter().collect(),
    };
    for solution in solutions {
        let solution = match solution {
            Ok(s) => s,
            Err(e) => {
                report
                    .failed
                    .push((rule.constraint.rule_id.clone(), e.to_string()));
                return;
            }
        };
        let Some(Term::NamedNode(this)) = solution.get("this") else {
            continue;
        };
        // The object a finding is about: the focus node where it is an element. A rule
        // over the whole model targets a placeholder node (`cim:IDchecks`) and names the
        // element in `?value`; with no element it is about the model, `global`, as the
        // hand-written rules report it.
        let element = |iri: &str| {
            datasets.iter().find_map(|ds| {
                cimsparql::iri::iri_to_mrid(iri, ds)
                    .map(|m| (m.clone(), ds.entries[&m].type_name().to_string()))
            })
        };
        let value = match solution.get("value") {
            Some(Term::NamedNode(v)) => Some(v.as_str()),
            _ => None,
        };
        let (object_id, class) = element(this.as_str())
            .or_else(|| value.and_then(element))
            .unwrap_or_else(|| ("global".to_string(), String::new()));
        let bindings: Vec<(String, String)> = solution
            .iter()
            .map(|(var, term)| (var.as_str().to_string(), lexical(term)))
            .collect();
        report.violations.push((
            file,
            Violation {
                object_id,
                rule_id: rule.constraint.rule_id.clone(),
                class,
                property: property(&rule.constraint.path),
                message: message(&rule.constraint, &bindings),
                severity: rule.constraint.severity.clone(),
                name: rule.constraint.name.clone(),
                description: rule.constraint.description.clone(),
            },
        ));
    }
}

/// A term as a message shows it: an IRI's text, a literal's value.
fn lexical(term: &Term) -> String {
    match term {
        Term::NamedNode(n) => n.as_str().to_string(),
        Term::Literal(l) => l.value().to_string(),
        other => other.to_string(),
    }
}

/// The path as the hand-written rules name a property: the last segment's local name.
fn property(path: &[String]) -> String {
    path.last()
        .map(|s| {
            let s = s.trim_start_matches('^');
            s.rsplit_once(':').map_or(s, |(_, local)| local).to_string()
        })
        .unwrap_or_default()
}

/// The constraint's message with `{$this}`, `{?value}` and the other `{?var}` filled in.
fn message(c: &SparqlConstraint, bindings: &[(String, String)]) -> String {
    let mut text = if c.message.is_empty() {
        c.name.clone()
    } else {
        c.message.clone()
    };
    for (var, value) in bindings {
        text = text
            .replace(&format!("{{?{var}}}"), value)
            .replace(&format!("{{${var}}}"), value);
    }
    text
}

// ── Preparing a query ────────────────────────────────────────────────────

/// The constraint's query, runnable: its prefixes declared, `$PATH` replaced by the
/// shape's path and `$this` by a variable bound to the shape's targets.
pub fn prepare(
    c: &SparqlConstraint,
    file_prefixes: &HashMap<String, String>,
) -> Result<String, String> {
    let expand = |name: &str| expand(name, file_prefixes);
    let mut prefixes: Vec<(String, String)> = c.query.prefixes.clone();
    let mut targets = Vec::new();
    for target in &c.targets {
        targets.push(match target {
            SparqlTarget::Class(class) => format!("{{ ?this a {} }}", expand(class)?),
            SparqlTarget::Node(node) => format!("{{ VALUES ?this {{ {} }} }}", expand(node)?),
            SparqlTarget::SubjectsOf(p) => {
                format!("{{ ?this {} ?cimoxide_object }}", expand(p)?)
            }
            SparqlTarget::ObjectsOf(p) => {
                format!("{{ ?cimoxide_subject {} ?this }}", expand(p)?)
            }
            SparqlTarget::Select(query) => {
                for (p, ns) in &query.prefixes {
                    match prefixes.iter().find(|(q, _)| q == p) {
                        Some((_, other)) if other != ns => {
                            return Err(format!(
                                "its target binds {p}: to {ns}, its query to {other}"
                            ));
                        }
                        Some(_) => {}
                        None => prefixes.push((p.clone(), ns.clone())),
                    }
                }
                format!("{{ {} }}", query.select.replace("$this", "?this"))
            }
        });
    }
    if targets.is_empty() {
        return Err("no targets".into());
    }
    let mut body = c.query.select.replace("$this", "?this");
    if body.contains("$PATH") {
        if c.path.is_empty() {
            return Err("$PATH without a path".into());
        }
        body = body.replace("$PATH", &path_expression(&c.path, file_prefixes)?);
    }
    let body = inject(&body, &targets.join(" UNION "))?;
    let prologue: String = prefixes
        .iter()
        .map(|(p, ns)| format!("PREFIX {p}: <{ns}>\n"))
        .collect();
    Ok(format!("{prologue}{body}"))
}

/// A prefixed name, or a `<full IRI>`, as a full IRI in angle brackets.
fn expand(name: &str, prefixes: &HashMap<String, String>) -> Result<String, String> {
    if name.starts_with('<') {
        return Ok(name.to_string());
    }
    let (prefix, local) = name
        .split_once(':')
        .ok_or_else(|| format!("not a prefixed name: {name}"))?;
    let ns = prefixes
        .get(prefix)
        .ok_or_else(|| format!("prefix {prefix}: is not declared"))?;
    Ok(format!("<{ns}{local}>"))
}

/// A shape's path as a SPARQL property path: segments in sequence, `^` for an inverse,
/// `|a|b` for alternatives.
fn path_expression(path: &[String], prefixes: &HashMap<String, String>) -> Result<String, String> {
    let segment = |s: &str| -> Result<String, String> {
        if let Some(alternatives) = s.strip_prefix('|') {
            let parts = alternatives
                .split('|')
                .map(|a| expand(a, prefixes))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(format!("({})", parts.join("|")))
        } else if let Some(inverse) = s.strip_prefix('^') {
            Ok(format!("^{}", expand(inverse, prefixes)?))
        } else {
            expand(s, prefixes)
        }
    };
    Ok(path
        .iter()
        .map(|s| segment(s))
        .collect::<Result<Vec<_>, _>>()?
        .join("/"))
}

/// `body` with `pattern` joined in at the start of its outermost group, so the group's
/// filters and `NOT EXISTS` see `?this` bound.
fn inject(body: &str, pattern: &str) -> Result<String, String> {
    let mut in_string: Option<char> = None;
    let mut in_comment = false;
    for (i, c) in body.char_indices() {
        if in_comment {
            in_comment = c != '\n';
            continue;
        }
        match in_string {
            Some(q) if c == q => in_string = None,
            Some(_) => {}
            None => match c {
                '"' | '\'' => in_string = Some(c),
                '#' => in_comment = true,
                '{' => {
                    return Ok(format!(
                        "{}{{ {{ {pattern} }} {}",
                        &body[..i],
                        &body[i + 1..]
                    ));
                }
                _ => {}
            },
        }
    }
    Err("no group to bind $this in".into())
}

/// The profile files [`cimschema::profile_file::PROFILES_ENV`] names, as [`SparqlRules`];
/// `None` when it is unset.
pub fn from_env() -> Option<Result<SparqlRules, String>> {
    Pool::from_env().map(SparqlRules::from_pool)
}

/// [`SparqlRules`] from explicit directories, searched as `CIMOXIDE_PROFILES` is.
pub fn from_dirs(dirs: &[PathBuf]) -> Result<SparqlRules, String> {
    SparqlRules::from_pool(&Pool::scan(dirs))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_is_bound_in_the_outermost_group() {
        let q = "SELECT $this WHERE { $this a ?t . FILTER NOT EXISTS { $this ?p ?o } }"
            .replace("$this", "?this");
        let out = inject(&q, "{ ?this a <urn:C> }").unwrap();
        assert_eq!(
            out,
            "SELECT ?this WHERE { { { ?this a <urn:C> } }  ?this a ?t . FILTER NOT EXISTS { ?this ?p ?o } }"
        );
    }

    #[test]
    fn a_path_becomes_a_property_path() {
        let prefixes = HashMap::from([("cim".to_string(), "urn:c#".to_string())]);
        let path = vec!["^cim:A.b".to_string(), "|cim:C.d|cim:E.f".to_string()];
        assert_eq!(
            path_expression(&path, &prefixes).unwrap(),
            "^<urn:c#A.b>/(<urn:c#C.d>|<urn:c#E.f>)"
        );
    }
}
