use std::{env, fs, path::PathBuf};

fn main() {
    let parts = [
        "src/parts/01.rs.part",
        "src/parts/02.rs.part",
        "src/parts/03.rs.part",
        "src/parts/04.rs.part",
        "src/parts/05.rs.part",
        "src/parts/06.rs.part",
        "src/parts/07.rs.part",
        "src/parts/08.rs.part",
        "src/parts/09.rs.part",
        "src/parts/10.rs.part",
    ];

    let mut source = String::new();

    for part in parts {
        println!("cargo:rerun-if-changed={part}");
        source.push_str(
            &fs::read_to_string(part)
                .unwrap_or_else(|error| panic!("Falha ao ler {part}: {error}")),
        );
    }

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR não definido"));
    fs::write(out_dir.join("magnifier_generated.rs"), source)
        .expect("Falha ao gerar o fonte consolidado");
}
