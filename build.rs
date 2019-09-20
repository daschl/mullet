use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo:rerun-if-changed={}/src/go/", manifest_dir);

    // /usr/local/opt/go\@1.11/bin/go build -buildmode=c-archive -o target/debug/deps/libquery.a main.go

    // Build query into c-archive
    let output = Command::new("/usr/local/opt/go@1.11/bin/go")
        .arg("build")
        .arg("-buildmode=c-archive")
        .arg("-o")
        .arg(format!("{}/libqueryengine.a", out_path.to_str().unwrap()))
        .arg(format!("{}/src/go/query.go", manifest_dir))
        .output()
        .expect("Cannot run go build");

    // Link query
    println!("cargo:rustc-link-search={}", out_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=queryengine");

    // Link other stuff needed for query
    println!("cargo:rustc-flags=-l curl");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
    println!("cargo:rustc-link-lib=framework=Security");
}