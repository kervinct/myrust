use deeply::nested::function as other_function;

fn function() {
    println!("called `function()`");
}

mod my {
    pub fn function() {
        println!("called `my::function()`");
    }

    fn private_function() {
        println!("called `my::private_function()`");
    }

    pub fn indirect_access() {
        print!("called `my::indirect_access()`, that\n> ");
        private_function();
    }

    pub fn indirect_call() {
        print!("called `my::indirect_call()`, that\n> ");
        function();
        {
            use cool::function as root_cool_function;

            print!("> ");
            root_cool_function();
        }

        {
            use self::cool::function as my_cool_function;

            print!("> ");
            my_cool_function();
        }

        {
            use super::function as root_function;
            print!("> ");
            root_function();
        }
    }

    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }

    pub mod nested {
        pub fn function() {
            println!("called `my::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my::nested::private_function()`");
        }
    }

    mod inaccessible {
        #[allow(dead_code)]
        pub fn public_function() {
            println!("called `my::inaccessible::public_function()`");
        }
    }
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

fn main() {
    function();

    println!("Hello world!");

    my::function();

    /*
     * visibility
     */
    my::function();
    function();
    my::indirect_access();
    my::nested::function();

    /*
     * `use` import
     */
    other_function();
    println!("Entering block");
    {
        use deeply::nested::function;
        function();
        println!("Leaving block");
    }

    function();

    /*
     * `super` and `self`
     */
    my::indirect_call();
}
