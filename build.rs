mod redoxri;
use redoxri::*;

fn main() -> Result<(), RxiError> {
    let _redoxri = Redoxri::new(&[""]);

    let out = Mcule::new("output", "out/")
        .add_step(&["mkdir", "out"])
        .compile();

    let test = Mcule::new("main", "out/main")
        .with(&[
            "src/main.rs".into(),
        ])
        .add_step(&[
            "rustc", "src/main.rs", "-o", "$out", "-Copt-level=3"
        ])
        .compile()
        .run();

    #[cfg(run)]
    let magick = Cmd::new("magick").arg("out/output.ppm").arg("out/output.png").status()?;

    #[cfg(run)]
    let feh = Cmd::new("gwenview").arg("out/output.png").status()?;


    Ok(())
}
