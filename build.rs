extern crate cc;


fn main() {
    cc::Build::new()
        .cpp(true)
        .file("src/saxx.cpp")
        .compile("saxx");
}
