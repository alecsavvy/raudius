fn main() {
    println!("cargo:rerun-if-changed=swagger.yaml");
    std::process::Command::new("./codegen.sh")
        .output()
        .expect("codegen failed");
}
