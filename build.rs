#[cfg(any(feature = "fbs", feature = "proto"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    const ARROW_TAG: &str = "apache-arrow-5.0.0";
    const ARROW_GIT: &str = "https://github.com/apache/arrow";

    use flate2::read::GzDecoder;
    use std::{env, path::PathBuf};
    use tar::Archive;

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    Archive::new(GzDecoder::new(
        reqwest::blocking::get(format!("{}/archive/{}.tar.gz", ARROW_GIT, ARROW_TAG))
            .expect("arrow download failed"),
    ))
    .unpack(&out_dir)
    .expect("arrow tarball extract failed");

    let arrow_dir = out_dir.join(format!("arrow-{}", ARROW_TAG));
    let arrow_dir = arrow_dir.display();

    #[cfg(feature = "fbs")]
    let fbs_files = [
        #[cfg(feature = "file")]
        "format/File.fbs",
        #[cfg(feature = "message")]
        "format/Message.fbs",
        #[cfg(feature = "schema")]
        "format/Schema.fbs",
        #[cfg(feature = "sparse_tensor")]
        "format/SparseTensor.fbs",
        #[cfg(feature = "tensor")]
        "format/Tensor.fbs",
        #[cfg(feature = "plasma_common")]
        "cpp/src/plasma/common.fbs",
        #[cfg(feature = "plasma")]
        "cpp/src/plasma/plasma.fbs",
        #[cfg(feature = "feather")]
        "cpp/src/arrow/ipc/feather.fbs",
    ]
    .iter()
    .map(|file| format!("{}/{}", arrow_dir, file))
    .collect::<Vec<_>>();

    #[cfg(feature = "fbs")]
    assert!(std::process::Command::new(flatc::flatc())
        .arg("--rust")
        // .arg("--gen-all")
        .arg("--filename-suffix")
        .arg("")
        .arg("-o")
        .arg(out_dir)
        .args(fbs_files)
        .status()?
        .success());

    #[cfg(feature = "proto")]
    let proto_files = [
        #[cfg(feature = "flight")]
        "format/Flight.proto",
    ]
    .iter()
    .map(|file: &&str| format!("{}/{}", arrow_dir, file))
    .collect::<Vec<_>>();

    #[cfg(feature = "proto")]
    prost_build::compile_protos(&proto_files, &[format!("{}/format", arrow_dir)])?;

    Ok(())
}

#[cfg(not(any(feature = "fbs", feature = "proto")))]
fn main() {}
