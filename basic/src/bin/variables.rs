fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    let _unused_variable = 3u32;
    let _noisy_unused_variable = 2u32;

    /*
     * mutability
     */
    let _immutable_variable = 1;
    let mut mutable_variable = 1;

    println!("Before mutation: {}", mutable_variable);

    mutable_variable += 1;

    println!("After mutation: {}", mutable_variable);

    //_immutable_variable += 1;

    /*
     * scope and shadowing
     */
    let long_lived_variable = 1;

    {
        let short_lived_variable = 2;
        println!("inner short: {}", short_lived_variable);

        let long_lived_variable = 5_f32;

        println!("inner long: {}", long_lived_variable);
    }
    // println!("outer short: {}", short_lived_variable);
    println!("outer long: {}", long_lived_variable);

    /*
     * declare first
     */
    let a_binding;
    {
        let x = 2;

        a_binding = x * x;
    }
    println!("a variable: {}", a_binding);

    let another_binding;

    // println!("another_binding: {}", another_binding);

    another_binding = 1;
    println!("another_binding: {}", another_binding);

    /*
     * freezing
     */
    let mut _mutable_integer = 7i32;

    {
        let _mutable_integer = _mutable_integer;

        // _mutable_integer = 50;
    }

    _mutable_integer = 3;
}