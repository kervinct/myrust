#![feature(box_syntax)]

fn division(dividend: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        panic!("division by zero");
    } else {
        dividend / divisor
    }
}

fn give_princess(gift: &str) {
    if gift == "snake" { panic!("AAAaaaaa!!!!"); }
    println!("I love {}s!!!!", gift);
}

fn main() {
    let _x = box 0i32;

    division(3, 0);

    println!("This point won't be reached!");

    give_princess("teddy bear");
    give_princess("snake");
}