#![crate_type = "lib"]
#![crate_name = "erty"]

pub fn public_function() {
    println!("Called erty's `public_function()`");
}

fn private_function() {
    println!("Called erty's `private_function()`");
}

pub fn indirect_access() {
    print!("Called erty's `private_function()`, that\n> ");

    private_function();
}