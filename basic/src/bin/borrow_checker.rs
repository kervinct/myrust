fn main() { // 'main starts ----------------------------------------------+
    let stack_integer: i32 = 5; // 'a starts ---------------------------+ |
    let boxed_integer = Box::new(4); // 'b starts --------------------+ | |
    //                                                                | | |
    let ref_to_box: &i32 = &*boxed_integer; // 'c starts -----------+ | | |
    //                                                              | | | |
    let ref_to_another_box: &i32 = {  // 'let 'd starts ----------+ | | | |
        let another_boxed_integer = Box::new(3); // 'e starts --+ | | | | |
        &*another_boxed_integer  //                             | | | | | |
    }; // 'e 'let end --------------------------------------------- | | | |
    //                                                              | | | |
    let invalid_dereference = *ref_to_another_box; //               | | | |
} // 'd 'c 'b 'a 'main end -----------------------------------------------+

// stack_integer -> 'a
// boxed_integer -> 'b
// ref_to_box -> 'c
// ref_to_another_box -> 'd
// another_boxed_integer -> 'e
// 'main and 'let are scope of blocks

// 'c < 'b -> valid
// 'd > 'e -> invalid