use std::fmt::{Debug, Display};
use std::ops::Add;
use std::marker::PhantomData;

struct Pair<T> {
    first: T,
    second: T,
}

fn swap<T>(pair: Pair<T>) -> Pair<T> {
    let Pair { first, second } = pair;
    Pair { first: second, second: first}
}

struct Tuple2<T, U>(T, U);

///

struct Tup(f64,);
struct GenTup<T>(T,);

impl Tup {
    fn value(&self) -> &f64 {
        let &Tup(ref val) = self;
        val
    }
}

impl<T> GenTup<T> {
    fn value(&self) -> &T {
        let &GenTup(ref val) = self;

        val
    }
}

/// 幻型，运行时不使用，只用于编译时静态检查
struct Tuple<A>(A,);

// A泛型+B隐藏参数，A会分配内存存储，B不会，因此B不能用于计算
#[derive(PartialEq)]
struct PhantomTuple<A, B>(A, PhantomData<B>);

#[derive(PartialEq)]
struct PhantomStruct<A, B> {
    first: A,
    phantom: PhantomData<B>,
}


#[derive(Debug, Copy, Clone)]
enum Inch {}
#[derive(Debug, Copy, Clone)]
enum Mm {}

#[derive(Debug, Copy, Clone)]
struct Length<Unit>(f64, PhantomData<Unit>);

impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;

    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        Length(self.0 + rhs.0, PhantomData)
    }
}

struct A;
// 在定义中，没有使用<A>，因此A是一个具体类型，即上面定义的
// 因此，Single也是一个具体类型
struct Single(A);

// T是泛型参数，因此SingleGen是泛型
struct SingleGen<T>(T);

struct S(A);
struct SGen<T>(T);

fn reg_fn(_s: S) {}
// 显式使用A作为类型参数，不是泛型参数
fn gen_spec_t(_s: SGen<A>) {}
fn gen_spec_i32(_s: SGen<i32>) {}
fn generic<T>(_s: SGen<T>) {}

struct Empty;
struct Null;

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

impl<T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {}
}

trait HasArea {
    fn area(&self) -> f64;
}
impl HasArea for Rectangle {
    fn area(&self) -> f64 { self.length * self.height }
}
#[derive(Debug)]
struct Rectangle { length: f64, height: f64 }
#[allow(dead_code)]
struct Triangle { length: f64, height: f64 }

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}
fn area<T: HasArea>(t: &T) -> f64 { t.area() }

struct Cardianl;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardianl {}
impl Blue for BlueJay {}

// 仅对实现trait的类型有效，与trait是否空无关
fn red<T: Red>(_: &T) -> &'static str { "red" }
fn blue<T: Blue>(_: &T) -> &'static str { "blue" }


fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: `{:?}`", t);
    println!("Display: `{}`", t);
}
fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: `{:?}`", t);
    println!("u: `{:?}`", u);
}

trait PrintInOption {
    fn print_in_option(self);
}

impl<T> PrintInOption for T where
    Option<T>: Debug {
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

struct Container(i32, i32);

trait Contains<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains<i32, i32> for Container {
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    fn first(&self) -> i32 { self.0 }
    fn last(&self) -> i32 { self.1 }
}

fn difference<A, B, C>(container: &C) -> i32 where C: Contains<A, B> {
    container.last() - container.first()
}

trait NewContains {
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl NewContains for Container {
    type A = i32;
    type B = i32;

    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    fn first(&self) -> i32 { self.0 }
    fn last(&self) -> i32 { self.1 }
}

fn new_difference<C: NewContains>(container: &C) -> i32 {
    container.last() - container.first()
}

fn main() {
    let pair_of_chars: Pair<char> = Pair { first: 'a', second: 'b' };  // Explicitly
    let pair_of_ints = Pair { first: 1i32, second: 2 };  // Implicitly
    let _tuple: Tuple2<char, i32> = Tuple2('R', 2);
    let _swapped_pair_of_chars = swap::<char>(pair_of_chars);  // Explicitly
    let _swapped_pair_of_ints = swap(pair_of_ints); // Implicitly

    let _s = Single(A);

    let _char: SingleGen<char> = SingleGen('a');
    let _t = SingleGen(A);
    let _i32 = SingleGen(6);
    let _char = SingleGen('a');

    /*
     * 泛型函数
     */
    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));
    generic::<char>(SGen('a'));  // 显式
    generic(SGen('c'));  // 隐式

    /*
     * Trait
     */
    let empty = Empty;
    let null = Null;
    empty.double_drop(null);

    /*
     * bounds
     */
    let rectangle = Rectangle { length: 3.0, height: 4.0 };
    let _triangle = Triangle { length: 3.0, height: 3.0 };
    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));

    // print_debug(*_triangle);
    // println!("Area: {}", area(*_triangle));

    let cardinal = Cardianl;
    let blue_jay = BlueJay;
    let _turkey = Turkey;

    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
    // println!("A turkey is {}", red(&_turkey));

    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];
    compare_prints(&string);
    compare_types(&array, &vec);

    vec.print_in_option();

    /*
     * 关联类型
     */
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
        &number_1,
        &number_2,
        Contains::contains(&container, &number_1, &number_2));
    println!("First number: {}", Contains::first(&container));
    println!("Last number: {}", NewContains::last(&container));
    println!("The difference is: {}", difference(&container));

    println!("The new difference is: {}", new_difference(&container));

    /*
     * implementation
     */
    let x = Tup(3.0);
    let y = GenTup(3i32);
    println!("{}, {}", x.value(), y.value());

    /*
     * phantom types
     */
    let _tuple: Tuple<char> = Tuple('R');

    // f32/f64是隐藏参数
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    let _struct1: PhantomStruct<char, f32> = PhantomStruct { first: 'Q', phantom: PhantomData };
    let _struct2: PhantomStruct<char, f64> = PhantomStruct { first: 'Q', phantom: PhantomData };

    // 编译错误，类型不匹配，无法比较
    // println!("_tuple1 == _tuple2 yields: {}", _tuple1 == _tuple2);
    // println!("_struct1 == _struct2 yields: {}", _struct1 == _struct2);

    /*
     * Unit conversion
     */
    let one_foot: Length<Inch> = Length(12.0, PhantomData);
    let one_meter: Length<Mm> = Length(1000.0, PhantomData);

    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    println!("one foot + one_foot = {:?} in", two_feet);
    println!("one meter + one_meter = {:?} mm", two_meters);
}