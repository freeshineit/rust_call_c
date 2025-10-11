fn main() {
    println!("cargo:rerun-if-changed=cpplibs/hello.cpp");

    let mut builder: cc::Build = cc::Build::new();
    builder
        .cpp(true)
        .file("./cpplibs/hello.cpp")
        .compile("hello");
    // panic!("target:{:?}",builder.target(target))
    // panic!("tools:{:?}", builder.get_compiler().path());
}