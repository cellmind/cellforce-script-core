use rust2go::RegenArgs;

fn main() {
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=Security");
        println!("cargo:rustc-link-lib=resolv");
    }


    rust2go::Builder::new()
        .with_go_src("./go")
        .with_regen_arg(RegenArgs {
            src: "./src/runner/go/interpreter.rs".into(),
            dst: "./go/gen.go".into(),
            go118: true,
            ..Default::default()
        })
        .build();
}
