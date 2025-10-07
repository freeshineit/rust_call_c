// #[link(name = "hello")] // 告诉编译器链接hello.o
unsafe extern { fn hello(); }

fn main() {
    unsafe { hello(); }
}