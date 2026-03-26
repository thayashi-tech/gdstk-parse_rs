// build.rs
fn main() {
    let gdstk_dir = std::path::PathBuf::from("gdstk");
    let gdstk_include = gdstk_dir.join("include");
    let gdstk_external = gdstk_dir.join("external");
    let gdstk_src_dir = gdstk_dir.join("src");
    let project_src = std::path::PathBuf::from("src");

    let mut b = autocxx_build::Builder::new(
        "src/lib.rs",
        &[&gdstk_include, &gdstk_external, &project_src],
    )
    .extra_clang_args(&["-std=c++17"])
    .build()
    .unwrap();

    let srcs = r#"
cell.cpp
curve.cpp
flexpath.cpp
gdsii.cpp
label.cpp
layername.cpp
library.cpp
oasis.cpp
polygon.cpp
property.cpp
raithdata.cpp
rawcell.cpp
reference.cpp
repetition.cpp
robustpath.cpp
style.cpp
utils.cpp
"#;

    for src in srcs.lines().map(|s| s.trim()).filter(|s| !s.is_empty()) {
        b.file(gdstk_src_dir.join(src));
    }
    b.file(gdstk_external.join("clipper").join("clipper.cpp"));
    b.cpp(true)
        .std("c++17")
        .flag_if_supported("-w")
        .compile("gdstk-rs");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/wrapper.h");
    println!("cargo:rustc-link-lib=qhull_r");
    println!("cargo:rustc-link-lib=z");
}
