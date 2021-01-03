#![feature(box_syntax)]
#![feature(box_patterns)]

fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("destroying box that contains {}", boxed_i32);
}

fn borrow_i32(borrowed_i32: &i32) {
    println!("This box contains {}", borrowed_i32);
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
struct Book {
    author: &'static str,
    title: &'static str,
    year: u32,
}

fn borrow_book(book: &Book) {
    println!("I borrowed {} {} edition", book.title, book.year);
}

fn new_edition(book: &mut Book) {
    book.year = 2014;
}

struct Point { x: i32, y: i32, z: i32 }

#[derive(Copy, Clone)]
struct RefPoint { x: i32, y: i32 }

fn main() {
    let boxed_i32 = Box::new(5);
    let stacked_i32 = 6_i32;

    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        let _ref_to_int: &i32 = &boxed_i32;

        // eat_box_i32(boxed_i32);
    }

    eat_box_i32(boxed_i32);

    /*
     * Mutability
     */
    let geb = Book {
        author: "Douglas Hofstadter",
        title: "Godel, Escher, Bach",
        year: 1979,
    };

    borrow_book(&geb);

    // new_edition(&mut geb);

    let mut mutable_geb = geb;

    new_edition(&mut mutable_geb);

    borrow_book(&mutable_geb);

    /*
     * Deprecated: Now rust can infered lifetime of ref
     * Freezing
     */
    let mut _integer = 5i32;
    {
        let _ref_to_integer = &_integer;

        _integer = 4;
    }
    _integer = 4;

    /*
     * Deprecated: Now rust can infered lifetime of ref
     * Aliasing
     */
    let mut point = Point { x: 0, y: 0, z: 0 };
    {
        let borrowed_point = &point;
        let another_borrow = &point;

        println!("Point has coordinates: ({}, {}, {})",
            borrowed_point.x, another_borrow.y, point.z);

        let _mutable_borrow = &mut point;
    }

    {
        let mutable_borrow = &mut point;

        mutable_borrow.x = 5;

        let _y = &point.y;

        println!("Point Z coordinate is {}", point.z);
    }

    println!("Point now has coordinates: ({}, {}, {})", point.x, point.y, point.z);

    /*
     * ref pattern
     */
    let c = 'Q';

    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    let point = RefPoint { x: 0, y: 0 };
    let _copy_of_x = {
        let RefPoint { x: ref ref_to_x, y: _ } = point;
        *ref_to_x
    };

    let mut mutable_point = point;
    {
        let RefPoint { x: _, y: ref mut mut_ref_to_y } = mutable_point;
        *mut_ref_to_y = 1;
    }
    println!("point is ({}, {})", point.x, point.y);
    println!("mutable_point is ({}, {})", mutable_point.x, mutable_point.y);

    let mut tuple = (box 5u32, 3u32);
    {
        let (box ref mut i, _) = tuple;
        *i = 3;
    }
    println!("tuple is {:?}", tuple);
}
