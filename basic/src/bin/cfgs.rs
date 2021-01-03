#[cfg(target_os="linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

#[cfg(not(target_os="linux"))]
fn are_you_on_linux() {
    println!("You are not running linux!");
}

// 只能在rustc编译时使用，rustc --cfg some_condition
// #[cfg(some_condition)]
// fn conditional_function() {
//     println!("condition met!");
// }

fn main() {
    are_you_on_linux();
    // conditional_function();

    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitly linux!");
    } else {
        println!("Yes. It's definitly not linux!");
    }
}