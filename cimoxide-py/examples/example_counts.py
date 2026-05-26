import os
import cimoxide

REALGRID = os.path.join(os.path.dirname(__file__), "..", "..", "CGMES-Test-Configurations", "v3.0", "RealGrid", "RealGrid-Merged")

ds = cimoxide.CimDataset.decode_files([
    os.path.join(REALGRID, "RealGrid_EQ.xml"),
    os.path.join(REALGRID, "RealGrid_SSH.xml"),
    os.path.join(REALGRID, "RealGrid_TP.xml"),
    os.path.join(REALGRID, "RealGrid_SV.xml"),
])

print(f"Total objects: {len(ds)} \n")

by_type = ds.by_type()
print(f"{'Type':<50} {'Count':>6}")
print("-" * 58)
for type_name, mrids in sorted(by_type.items(), key=lambda x: -len(x[1])):
    print(f"{type_name:<50} {len(mrids):>6}")
