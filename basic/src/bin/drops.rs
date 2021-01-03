struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

fn main() {
    let _a = Droppable { name: "a" };

    {
        let _b = Droppable { name: "b" };

        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };

            println!("Existing block B");
        }
        println!("Just existed block B");

        println!("Existing block A");
    }
    println!("Just existed block A");

    drop(_a);

    println!("end of the main function");
}