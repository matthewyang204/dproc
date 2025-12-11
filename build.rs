fn link() {
    println!("cargo:rustc-link-search=native=obj");
    println!("cargo:rustc-link-lib=static=mysolvers");
    println!("cargo:rustc-link-search=native=obj");
    println!("cargo:rustc-link-lib=static=charge");
}

fn main() {
    link();
}
