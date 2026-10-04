//! NC validation throughput.
//!
//! Unlike the decoder's A/B, there is no prior implementation to compare
//! against — NC had no validation at all. The number to establish is whether
//! interpreting ~14,800 checks over a real-sized dataset is in the same range
//! as the generated CGMES validators or an order off. If it is close, that is
//! evidence the 260,900 generated lines could become a table too; acting on
//! that is a separate decision.
//!
//! `sh:inversePath` is measured separately because it is the only part of the
//! design with a complexity question: answering "how many things point at me"
//! needs a reverse index over the whole dataset, built once, rather than a
//! forward field read.

use std::fmt::Write;

use cimdecoder::CimDataset;
use cimstructs::base::{AttrKind, ClassDef};
use cimvalidation::{validate_nc_profile, Config};
use criterion::{criterion_group, criterion_main, Criterion, Throughput};

const ELEMENTS: usize = 20_000;

const NS_NC: &str = "https://cim4.eu/ns/nc#";
const NS_CIM: &str = "https://cim.ucaiug.io/ns#";

fn prefix_for(ns: &str) -> Option<&'static str> {
    match ns {
        NS_NC => Some("nc"),
        NS_CIM => Some("cim"),
        _ => None,
    }
}

/// Classes the CO profile actually has shapes for.
///
/// Benchmarking classes no shape targets would measure the `by_type` lookups
/// and nothing else — the interpreter's cost is per *target*, not per element.
fn co_target_classes() -> Vec<&'static str> {
    let mut names: Vec<&'static str> = cimvalidation::nc_shapes()
        .iter()
        .filter(|s| s.profiles.contains(&"CO"))
        .flat_map(|s| s.targets.iter())
        .flat_map(|t| match t {
            cimvalidation::shapes::Target::Class(classes) => classes.to_vec(),
            cimvalidation::shapes::Target::SubjectsOf(_) => Vec::new(),
        })
        .collect();
    names.sort_unstable();
    names.dedup();
    names
}

/// Synthesise a CO dataset. No NC corpus ships with the repo, and the only
/// fixture has four elements.
///
/// Every element is deliberately *incomplete*, so the cardinality rules fire.
/// A document that satisfied everything would benchmark the empty path — which
/// is precisely the mistake the decoder bench's full-decode assertion caught
/// once already.
fn co_document(n: usize) -> String {
    let targets = co_target_classes();
    let classes: Vec<&ClassDef> = cimstructs::nc_classes::CLASSES
        .iter()
        .filter(|c| c.concrete && prefix_for(c.ns).is_some())
        .filter(|c| targets.contains(&c.qualified))
        .collect();
    assert!(!classes.is_empty(), "no CO-targeted concrete classes found");

    let mut out = String::with_capacity(n * 200);
    out.push_str(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"\n\
         \x20        xmlns:cim=\"https://cim.ucaiug.io/ns#\"\n\
         \x20        xmlns:nc=\"https://cim4.eu/ns/nc#\">\n",
    );

    for i in 0..n {
        let class = classes[i % classes.len()];
        let cp = prefix_for(class.ns).expect("filtered above");
        let mrid = format!("urn:uuid:{i:08x}-0000-0000-0000-000000000000");
        writeln!(out, "  <{cp}:{local} rdf:about=\"{mrid}\">", local = class.local).unwrap();
        writeln!(
            out,
            "    <cim:IdentifiedObject.mRID>{i:08x}-0000-0000-0000-000000000000</cim:IdentifiedObject.mRID>"
        )
        .unwrap();
        // A couple of literals, so the datatype and nodeKind checks have
        // something to read, and one association so the ref-type rules do.
        for attr in class
            .attrs
            .iter()
            .filter(|a| a.kind == AttrKind::Literal && prefix_for(a.ns).is_some())
            .take(2)
        {
            let ap = prefix_for(attr.ns).expect("filtered above");
            writeln!(out, "    <{ap}:{id}>1</{ap}:{id}>", id = attr.id).unwrap();
        }
        if let Some(attr) = class
            .attrs
            .iter()
            .find(|a| a.kind == AttrKind::Association && prefix_for(a.ns).is_some())
        {
            let ap = prefix_for(attr.ns).expect("filtered above");
            let target = format!("urn:uuid:{:08x}-0000-0000-0000-000000000000", (i + 1) % n);
            writeln!(
                out,
                "    <{ap}:{id} rdf:resource=\"{target}\"/>",
                id = attr.id
            )
            .unwrap();
        }
        writeln!(out, "  </{cp}:{local}>", local = class.local).unwrap();
    }

    out.push_str("</rdf:RDF>\n");
    out
}

fn bench(c: &mut Criterion) {
    let doc = co_document(ELEMENTS);
    let ds = CimDataset::decode_str(&doc).expect("synthetic document did not decode");
    assert_eq!(
        ds.entries.len(),
        ELEMENTS,
        "synthetic document did not fully decode"
    );

    let cfg = Config::default();

    // The load-bearing assertion. A resolution mistake produces rules that run
    // and match nothing, so "it completed" is not evidence that anything was
    // checked.
    let probe = validate_nc_profile(&ds, "CO", &cfg);
    assert!(
        probe.len() > ELEMENTS / 10,
        "only {} violations over {ELEMENTS} elements — the shapes are not firing, \
         so this would benchmark the empty path",
        probe.len()
    );
    eprintln!(
        "benchmarking {} violations over {ELEMENTS} elements ({} bytes)",
        probe.len(),
        doc.len()
    );

    let mut group = c.benchmark_group("nc_validate");
    group.throughput(Throughput::Elements(ELEMENTS as u64));
    group.bench_function("co_profile", |b| {
        b.iter(|| validate_nc_profile(&ds, "CO", &cfg))
    });
    group.finish();

    // A profile with no shapes for this data: the floor, i.e. what the target
    // lookups alone cost.
    let mut group = c.benchmark_group("nc_validate_floor");
    group.throughput(Throughput::Elements(ELEMENTS as u64));
    group.bench_function("unmatched_profile", |b| {
        b.iter(|| validate_nc_profile(&ds, "SM", &cfg))
    });
    group.finish();

    // Every NC profile over the same data, which is what `validate_files`
    // does when a dataset declares several.
    let mut group = c.benchmark_group("nc_validate_all_profiles");
    group.throughput(Throughput::Elements(ELEMENTS as u64));
    group.bench_function("18_profiles", |b| {
        b.iter(|| {
            let mut total = 0;
            for profile in cimvalidation::nc_profile_index().1 {
                total += validate_nc_profile(&ds, profile, &cfg).len();
            }
            total
        })
    });
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
