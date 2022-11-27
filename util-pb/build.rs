fn main() {
    tonic_build::configure()
        .out_dir("src/pb")
        .compile(&["./proto/calculate.proto", "./proto/echo.proto"], &["."])
        .unwrap();

    std::process::Command::new("cargo")
        .arg("fmt")
        .arg("--all")
        .output()
        .unwrap();

    println!("cargo:rerun-if-changed=proto/user.proto");
    println!("cargo:rerun-if-changed=build.rs");
}
