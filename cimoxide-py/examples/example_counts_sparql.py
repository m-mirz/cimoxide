"""Counting with SPARQL — the query-based counterpart to example_counts.py.

example_counts.py reads the decoder's own by_type() index, which can only count whole
classes. SPARQL can also count what by_type() cannot express: filtered attribute values and
joins across associations.
"""

import os
import cimoxide

REALGRID = os.path.join(os.path.dirname(__file__), "..", "..", "CGMES-Test-Configurations", "v3.0", "RealGrid", "RealGrid-Merged")

ds = cimoxide.CimDataset.decode_files([
    os.path.join(REALGRID, f"RealGrid_{p}.xml") for p in ("EQ", "SSH", "TP", "SV")
])
print(f"Total objects: {len(ds)}\n")

# 1. Count per class. Same answer as by_type(), but computed from the RDF graph.
rows = ds.query("""
    SELECT ?type (COUNT(?s) AS ?n)
    WHERE { ?s a ?type }
    GROUP BY ?type
    ORDER BY DESC(?n)
    LIMIT 10
""")

by_type = ds.by_type()
print(f"{'Type':<40} {'SPARQL':>8} {'by_type':>8}")
print("-" * 58)
for row in rows:
    name = row["type"].rsplit("#", 1)[-1]
    print(f"{name:<40} {row['n']:>8} {len(by_type.get(name, [])):>8}")

# 2. A filter on an attribute value — by_type() cannot express this.
[row] = ds.query("""
    SELECT (COUNT(*) AS ?n)
    WHERE { ?s a cim:ACLineSegment ; cim:ACLineSegment.r ?r . FILTER(?r > 1.0) }
""")
print(f"\nACLineSegments with r > 1.0 ohm: {row['n']}")

# 3. A join across an association — also out of reach for a per-class index.
[row] = ds.query("""
    SELECT (COUNT(*) AS ?n)
    WHERE {
      ?t a cim:Terminal ; cim:Terminal.ConductingEquipment ?e .
      ?e a cim:ACLineSegment .
    }
""")
print(f"Terminals attached to an ACLineSegment: {row['n']}")
