//! defining calling returning
/*
 *  fn name(param1: type1, ...) -> return_type {
 *      ...body...
 *  }
 */
fn next_birthday(name: &str, current_age: u8) {
    let next_age = current_age + 1;
    println!("{} next birthday is {}", name, next_age);
}

/*
 * name(arg1, ...);
 */

/*
 * returning
 */
fn square(num: i32) -> i32 {
    num * num
}

fn main() {
    next_birthday("Jake", 32);
    print!("The anwser is {}", square(3));
}