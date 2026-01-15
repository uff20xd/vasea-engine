mod redoxri;
use redoxri::*;

const COMMON_FLAGS: &[&str] = &["-Copt-level=3", "--edition=2024", "-Awarnings"];

fn main() -> Result<(), RxiError> {
    let redoxri = Redoxri::new(&[""]);

    let out = Mcule::new("output", "out/")
        .add_step(&["mkdir", "out"])
        .compile();

    let mut lib = Mcule::new("vasea", "out/libvasae.rlib")
        .with(&[
            "src/lib.rs".into(),
        ]);
    lib = lib.clone()
        .add_step(&[
            "rustc", &lib.inputs[0].outpath, "-o", "$out", "--crate-type=lib"
        ])
        .with_flags(COMMON_FLAGS)
        .compile();


    let mut test = Mcule::new("tests", "out/test")
        .with(&[
            "tests/tests.rs".into(), lib.clone(),
        ]);
    _ = test.clone()
        .add_step(&[
            "rustc", &test.inputs[0].outpath, "-o", "$out", "--extern", &(lib.name.clone() + "=" + &lib.outpath),
        ])
        .with_flags(COMMON_FLAGS)
        .compile()
        .run();

    if redoxri.flag_is_active("run") {
        let _magick = Cmd::new("magick").arg("out/output.ppm").arg("out/output.png").status()?;
        let _magick = Cmd::new("magick").arg("out/output.png").arg("out/output.jpg").status()?;
        let _feh = Cmd::new("feh").arg("out/output.jpg").status()?;
    }


    Ok(())
}
