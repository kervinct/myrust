fn apply<F>(f: F) where F: FnOnce() {
    f();
}

fn apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}


fn call_me<F: Fn()>(f: F) {
    f();
}

fn function() {
    println!("I'm a function");
}

fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();
    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    move || println!("This is a: {}", text)
}

fn main() {
    let captured_value = 7u32;

    let closure = |argument| {
        println!("I captured this: {}", captured_value);
        println!("Argument passed was: {}", argument);

        true
    };

    println!("Closure returned: {}", closure("a string"));

    fn function(i: i32) -> i32 { i + 1 }

    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;

    let i = 1;

    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    let one = || 1;
    println!("closure returning one: {}", one());

    /*
     * as input parameter
     */
    use std::mem;

    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        // 仅使用，不变引用，需要Fn
        println!("I said {}.", greeting);

        // 做修改，可变引用，需要FnMut
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // 手动释放，值，需要FnOnce
        mem::drop(farewell);
    };

    apply(diary);

    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));

    /*
     * input function
     */
    let x = 7;
    let print = || println!("{}", x);

    apply(print);

    let closure = || println!("I'm a closure!");
    call_me(closure);
    call_me(self::function);

    /*
     * as output parameter
     */
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();

    /*
     * iterator any/find
     */
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();

    // vector iter() 输出&T，可以模式匹配
    println!("2 in vec1: {}", vec1.iter().any(|&x| x==2));
    // iter() 输出&T，find的闭包的参数必须是&T，所以使用&&模式匹配
    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x==2));
    // vector into_iter() 输出T，不需要模式匹配
    // println!("2 in vec2: {}", vec2.into_iter().any(|x| x==2));
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x==2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // array iter()和into_iter() 均输出&T，需要模式匹配
    println!("2 in array1: {}", array1.iter().any(|&x| x==2));
    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x==2));
    println!("2 in array2: {}", array2.into_iter().any(|&x| x==2));
    println!("Find 2 in array2: {:?}", array2.into_iter().find(|&&x| x==2));
}