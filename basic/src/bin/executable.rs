//! rustc -L . executable.rs
//! `-L .` add current directory to the library search path
extern crate erty;

fn main() {
    erty::public_function();

    erty::indirect_access();
}