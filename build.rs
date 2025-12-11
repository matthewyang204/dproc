use cc;

fn build() {
    cc::Build::new()
        .file("lib/libcharge.c")
        .include("lib")
        .compile("charge");

    cc::Build::new()
        .file("lib/libmysolvers.c")
        .include("lib")
        .compile("mysolvers");
}

fn link() {
    println!("cargo:rustc-link-search=native=obj");
    println!("cargo:rustc-link-lib=static=mysolvers");
    println!("cargo:rustc-link-search=native=obj");
    println!("cargo:rustc-link-lib=static=charge");
}

fn main() {
    build();
    link();
}
