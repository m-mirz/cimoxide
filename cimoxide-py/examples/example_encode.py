import os
import sys
import tempfile

import cimoxide

REALGRID = os.path.join(os.path.dirname(__file__), "..", "..", "CGMES-Test-Configurations", "v3.0", "RealGrid", "RealGrid-Merged")
PROFILES = ["EQ", "SSH", "TP", "SV"]

ds = cimoxide.decode_files([os.path.join(REALGRID, f"RealGrid_{p}.xml") for p in PROFILES])
print(f"Decoded {len(ds)} objects from {len(PROFILES)} RealGrid profile files.\n")

out_dir = sys.argv[1] if len(sys.argv) > 1 else tempfile.mkdtemp(prefix="cimoxide-realgrid-")
ds.write_xml_files(out_dir, PROFILES)

print(f"Encoded back to {len(PROFILES)} profile files in {out_dir}:")
for p in PROFILES:
    path = os.path.join(out_dir, f"{p}.xml")
    print(f"  {path} ({os.path.getsize(path):,} bytes)")

# Sanity check: re-decoding the freshly encoded files should recover every object.
ds2 = cimoxide.decode_files([os.path.join(out_dir, f"{p}.xml") for p in PROFILES])
status = "OK" if len(ds2) == len(ds) else "MISMATCH"
print(f"\nRe-decoded {len(ds2)} objects from the encoded files (round-trip {status}).")
