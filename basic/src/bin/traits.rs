use std::iter;
use std::vec::IntoIter;

trait Animal {
    fn new(name: &'static str) -> Self where Self: Sized;

    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

struct Dog { name: &'static str }

impl Dog {
    fn wag_tail(&self) {
        println!("{} wag tail", self.name);
    }
}

impl Animal for Dog {
    fn new(name: &'static str) -> Dog {
        Dog { name: name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        "woof!"
    }

    fn talk(&self) {
        self.wag_tail();

        println!("{} says {}", self.name(), self.noise());
    }
}

struct Sheep { naked: bool, name: &'static str }

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already naked!", self.name());
        } else {
            println!("{} gets a haircut", self.name());

            self.talk();
            self.naked = true;
        }
    }
}

impl Animal for Sheep {
    fn new(name: &'static str) -> Sheep {
        Sheep { naked: false, name: name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaah"
        } else {
            "baaaaaaaaaaaaah"
        }
    }
}

#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;
        Centimeters(inches as f64 * 2.54)
    }
}

struct Seconds(i32);

fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep { naked: false, name: "dolly" })
    } else {
        Box::new(Dog { name: "Cook" })
    }
}

#[allow(dead_code)]
fn combine_vecs_explicit_return_type(
    v: Vec<i32>,
    u: Vec<i32>,
) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn combine_vecs(
    v: Vec<i32>,
    u: Vec<i32>,
) -> impl Iterator<Item=i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    move |x: i32| x + y
}

fn main() {
    let mut dolly: Sheep = Animal::new("Dolly");
    let spike: Dog = Animal::new("Spike");

    dolly.shear();

    spike.talk();
    dolly.talk();

    /*
     * Derive
     */
    let _one_second = Seconds(1);
    // println!("One second looks like: {}", _one_second); // error

    let foot = Inches(12);

    println!("One foot == {:?}", foot);

    let meter = Centimeters(100.0);

    let cmp =
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };
    println!("one foot is {} than one meter", cmp);

    /*
     * dyn
     */
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());

    /*
     * impl Trait
     */
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5];
    let mut v3 = combine_vecs(v1, v2);
    assert_eq!(Some(1), v3.next());
    assert_eq!(Some(2), v3.next());
    assert_eq!(Some(3), v3.next());
    assert_eq!(Some(4), v3.next());
    assert_eq!(Some(5), v3.next());
    println!("all done");

    let plus_one = make_adder_function(1);
    assert_eq!(plus_one(2), 3);
}