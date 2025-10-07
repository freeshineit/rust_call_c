fn main() {
    println!("cargo:rerun-if-changed=clibs/hello.cpp");

    let mut builder: cc::Build = cc::Build::new();
    builder
        .cpp(true)
        .file("./clibs/hello.cpp")
        .compile("hello");
    // panic!("target:{:?}",builder.target(target))
    // panic!("tools:{:?}", builder.get_compiler().path());
}