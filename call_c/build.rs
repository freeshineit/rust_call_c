fn main() {
    println!("cargo:rerun-if-changed=clibs/hello.c");

    let mut builder: cc::Build = cc::Build::new();
    builder
        .file("./clibs/hello.c")
        .shared_flag(false)
        .compile("hello");
    // panic!("target:{:?}",builder.target(target))
    // panic!("tools:{:?}", builder.get_compiler().path());
}