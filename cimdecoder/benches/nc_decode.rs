//! Decode throughput for NC elements, generated class table vs RDFS-loaded.
//!
//! The registry is memoized once per process, so the two tables cannot be
//! compared in a single run. This is the same binary run twice with the
//! environment differing, compared through criterion baselines — see
//! `scripts/bench_schema_source.sh`, which drives both halves back to back.
//!
//! The hypothesis under test is that this is *flat*: once the table exists,
//! both paths do identical hash lookups and the only per-element difference is
//! a dispatch branch.

use std::fmt::Write;

use cimdecoder::CimDataset;
use cimstructs::base::{AttrKind, ClassDef};
use criterion::{criterion_group, criterion_main, Criterion, Throughput};

const ELEMENTS: usize = 50_000;

const NS_NC: &str = "https://cim4.eu/ns/nc#";
const NS_CIM: &str = "https://cim.ucaiug.io/ns#";

/// NC declares classes in seven namespaces; the document below binds two.
/// Anything else would be emitted under the wrong prefix and silently dropped.
fn prefix_for(ns: &str) -> Option<&'static str> {
    match ns {
        NS_NC => Some("nc"),
        NS_CIM => Some("cim"),
        _ => None,
    }
}

/// Synthesise an NC document. No NC corpus ships with the repo — the only
/// fixture has three elements, which measures nothing.
///
/// Built from the *generated* table deliberately, so both halves of the A/B
/// decode byte-identical input.
fn nc_document(n: usize) -> String {
    let classes: Vec<&ClassDef> = cimstructs::nc_classes::CLASSES
        .iter()
        .filter(|c| c.concrete && prefix_for(c.ns).is_some())
        .filter(|c| {
            c.attrs
                .iter()
                .any(|a| a.kind == AttrKind::Literal && prefix_for(a.ns).is_some())
        })
        .collect();
    assert!(!classes.is_empty(), "no concrete NC classes with literals");

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
        writeln!(
            out,
            "  <{cp}:{local} rdf:about=\"urn:uuid:{i:08x}-0000-0000-0000-000000000000\">",
            local = class.local
        )
        .unwrap();
        for attr in class
            .attrs
            .iter()
            .filter(|a| a.kind == AttrKind::Literal && prefix_for(a.ns).is_some())
            .take(3)
        {
            let ap = prefix_for(attr.ns).expect("filtered above");
            writeln!(out, "    <{ap}:{id}>1</{ap}:{id}>", id = attr.id).unwrap();
        }
        writeln!(out, "  </{cp}:{local}>", local = class.local).unwrap();
    }

    out.push_str("</rdf:RDF>\n");
    out
}

fn bench(c: &mut Criterion) {
    let doc = nc_document(ELEMENTS);

    // Fail loudly rather than silently measuring a document that decodes to
    // nothing, which is what a prefix or class-name mistake would produce.
    let probe = CimDataset::decode_str(&doc).unwrap();
    assert_eq!(probe.entries.len(), ELEMENTS, "synthetic NC document did not fully decode");

    let mut group = c.benchmark_group("nc_decode");
    group.throughput(Throughput::Bytes(doc.len() as u64));
    group.bench_function("50k_elements", |b| {
        b.iter(|| CimDataset::decode_str(&doc).unwrap())
    });
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
