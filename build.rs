fn main() {
     println!("cargo::rerun-if-changed=./script.ld");
     println!("cargo::rerun-if-changed=./src/init.S");
     println!("cargo::rustc-link-arg-bins=-T./script.ld");
}
