use std::fmt::Debug;

#[derive(Debug)]
struct Triplet {
    one: i32,
    two: i32,
    three: i32,
}

impl Triplet {
    fn mut_one(&mut self) -> &mut i32 {
        &mut self.one
    }

    // fn mut_two<'structure, 'field>(&'structure mut self) -> &'field mut i32 {
    //     &mut self.two
    // }

    fn mut_three<'s>(&'s mut self) -> &'s mut i32 {
        &mut self.three
    }
}

struct Pair<'a, 'b> {
    one: &'a mut i32,
    two: &'b mut i32,
}

// 入参是两个借用，因为没有返回，编译器可以在编译时推断这两个借用是否超过函数调用的lifetime
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

// 函数参数中没有引用，a在编译时推断为函数自身lifetime即'static
fn failed_borrow<'a>() {
    let _x = 12;

    // 编译失败，不能限制内部变量的lifetime超过函数自身的lifetime
    // let y: &'a i32 = &_x;
}

fn print_one<'a>(x: &'a i32) {
    println!("`print_one`: x is {}", x);
}
fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}
fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("`print_multi`: x is {}, y is {}", x, y);
}
fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 { x }
// fn invlaid_output<'a>() -> &'a String { &String::from("foo") }

struct Owner(i32);
impl Owner {
    fn add_one<'a>(&'a mut self) { self.0 += 1; }
    fn print<'a>(&'a self) {
        println!("`print`: {}", self.0);
    }
}

#[derive(Debug)]
struct Borrowed<'a>(&'a i32);
#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}
#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

impl<'a> Default for Borrowed<'a> {
    fn default() -> Self {
        Self(&10)
    }
}

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
fn print<T>(t: T) where T: Debug {
    println!("`print`: t is {:?}", t);
}
fn print_ref<'a, T>(t: &'a T) where T: Debug + 'a {
    println!("`print_ref`: t is {:?}", t);
}

fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

fn elided_input(x: &i32) {
    println!("`elided_input`: {}", x);
}
fn annotated_input<'a>(x: &'a i32) {
    println!("`annotated_input`: {}", x);
}
fn elided_pass(x: &i32) -> &i32 { x }
fn annotated_pass<'a>(x: &'a i32) -> &'a i32 { x }

fn main() {
    /*
     * 显式标注
     */
    let i = 3;
    {
        let borrow1 = &i;
        println!("borrow1: {}", borrow1);
    }
    {
        let borrow2 = &i;
        println!("borrow2: {}", borrow2);
    }

    let (four, nine) = (4, 9);
    print_refs(&four, &nine);
    failed_borrow();

    /*
     * 函数
     */
    let x = 7;
    let y = 9;
    print_one(&x);
    print_multi(&x, &y);
    let z = pass_x(&x, &y);
    print_one(z);
    let mut t = 3;
    add_one(&mut t);
    print_one(&t);

    /*
     * 方法
     */
    let mut owner = Owner(18);
    owner.add_one();
    owner.print();

    let mut triplet = Triplet { one: 1, two: 2, three: 3 };

    println!("Before: {:?}", triplet);

    *triplet.mut_one() = 0;
    println!("After: {:?}", triplet);

    *triplet.mut_three() = 0;
    println!("After: {:?}", triplet);

    /*
     * Struct
     */
    let mut one = 1;
    {
        let mut two = 2;
        println!("Before: ({}, {})", one, two);

        let pair = Pair { one: &mut one, two: &mut two };

        *pair.one = 2;
        *pair.two = 1;

        println!("After: ({}, {})", pair.one, pair.two);
    }

    let x = 18;
    let y = 15;
    let single = Borrowed(&x);
    let double = NamedBorrowed{ x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is not borrowed in {:?}", number);

    /*
     * Trait
     */
    let b: Borrowed = Default::default();
    println!("b is {:?}", b);

    /*
     * 绑定
     */
    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);

    /*
     * 转换
     */
    let first = 2;
    {
        let second = 3;
        println!("The product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    }

    /*
     * 省略
     */
    let x = 3;
    elided_input(&x);
    annotated_input(&x);
    println!("`elided_pass`: {}", elided_pass(&x));
    println!("`annotated_pass`: {}", annotated_pass(&x));
}