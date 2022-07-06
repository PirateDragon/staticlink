
extern crate libexample;

fn main() {
    unsafe {
        libexample::example::test();
        // example::test();
    }
    println!("Hello, world!");
}
