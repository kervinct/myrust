/// never return => diverging

fn foo() -> {
    panic!("This call never returns.");
}

#![feature(never_type)]

fn main() {
    // let x: ! = panic!("This call never returns");
    // println!("You will never see this line!");

    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            let addition: u32 =
                match i%2 == 1 {
                    true => i,
                    // continue不返回u32，准确说是从不返回，因此不违反match语句的类型要求
                    false => continue,
                };
            acc += addition;
        }
        acc
    }

    println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
}