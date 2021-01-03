static LANGUAGE: &'static str = "Rust";
static THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn main() {
    let n = 16;

    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "smal" });

    // THRESHOLD = 5;

    {
        let _static_string: &'static str = "In read-only memory";
    }
}