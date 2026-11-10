fn main() {
    println!("cargo:rerun-if-changed=instagram_res.o");
    println!("cargo:rustc-link-arg=instagram_res.o");
}
