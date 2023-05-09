from os import listdir, system
from os.path import isfile, join

for f in [f for f in listdir("examples/") if isfile(join("examples/", f))]:
    new_name = f.replace(".yuasm", "")

    print(f"=== Compiling {f} ===")
    system(f"cargo run -- assemble -i examples/{f} -o examples/compiled/{new_name}.bin")

    