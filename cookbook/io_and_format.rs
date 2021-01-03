use std::io::{self, Write};
use std::f64;

fn main() {
    println!("Let's print some lines:");
    println!();
    println!("Hello, world!");
    println!("{}, {}!", "Hello", "world");
    print!("Hello, ");
    println!("world!");
    println!("Arguments can be referred to by their positions: {0}, {1} and {1}, {0}! are built from the same arguments", "Hello", "world");
    println!("Furthermore the arguments can be named: \"{greeting}, {object}!\"", greeting="Hello", object="world");

    println!("Number formating: Pi is {0:.3} or {0:.0} for short", f64::consts::PI);

    println!("... and there is more: {0:>0width$}={0:>width$}={0:#x}", 1535, width=5);

    let _ = write!(&mut io::stdout(), "Underneath, it's all writing to a tream...");
    println!();

    println!("Write something!");
    let mut input = String::new();
    if let Ok(n) = io::stdin().read_line(&mut input) {
        println!("You wrote: {} ({} bytes)", input, n);
    } else {
        eprintln!("There was an error.");
    }
}