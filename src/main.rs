


use structopt::StructOpt;
use std::path::PathBuf;
use libexample::example::test;

#[derive(StructOpt, Debug)]
#[structopt(name = "loading shared library")]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    shared: PathBuf,
}

fn main() {
    unsafe {
        libexample::example::test();
        test();
        // example::test();
    }
    let opt = Opt::from_args();

        unsafe {
            let lib = libloading::Library::new(
              opt.shared
            ).unwrap();
            
            let func: libloading::Symbol<unsafe extern "C" fn() -> i32  > =
              lib.get(b"test").unwrap();
            func();
          }
    
    println!("Hello, world!");
}
