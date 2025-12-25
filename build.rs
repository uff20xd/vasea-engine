mod redoxri;
use redoxri::*;

fn main() -> Result<(), RxiError> {
    let _redoxri = Redoxri::new(&[""]);

    let out = Mcule::new("output", "out/")
        .add_step(&["mkdir", "out"])
        .compile();

    let ks_src = Mcule::new("karottenschaeler_src", "libs/libkarottenschaeler/lib.rs");

    let karottenschaeler = Mcule::new("karottenschaeler", "out/libkarottenschaeler.rlib")
        .with(&[
            ks_src.clone(),
        ])
        .add_step(&[
            "rustc", &ks_src.outpath, "-o" , "$out", "--crate-type", "lib",
            "-Clink-args=-lc",
            //"--extern", &("libc".to_owned() + "=" + &libc.outpath),
        ])
        .mute()
        .compile();

    let test = Mcule::new("main", "out/main")
        .with(&[
            karottenschaeler.clone(),
            "src/main.rs".into(),
        ])
        .add_step(&[
            "rustc", "src/main.rs", "-o", "$out",
            "--extern", &(karottenschaeler.name + "=" + &karottenschaeler.outpath),
        ])
        .compile()
        .run();

    Ok(())
}
