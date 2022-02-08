mod model {

    use std::fmt::Display;

    enum Fruit {
        Apple,
        Banana,
        Kiwi,
    }

    impl Display for Fruit {
        // fn display(&self) -> String {
        //     match self {
        //         Fruit::Apple => "an apple".to_owned(),
        //         Fruit::Banana => "a banana".to_owned(),
        //         Fruit::Kiwi => "a kiwi".to_owned(),
        //     }
        // }
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                Fruit::Apple => f.write_str("an apple"),
                Fruit::Banana => f.write_str("a banana"),
                Fruit::Kiwi => f.write_str("a kiwi"),
            }
        }
    }

    // impl<T: Displayable> Option<T> {
    pub fn show_item<T: Display>(item: &Option<T>, which: &str) {
        match item {
            Some(what) => {
                println!("{} hand is holding {}", which, what)
            }
            _ => {
                println!("{} hand is not holding anything", which)
            }
        }
    }
    // }

    pub struct Hands {
        left: Option<Fruit>,
        right: Option<Fruit>,
    }

    impl Hands {
        pub fn show_hands(&self) {
            // We can borrow hands instead preventing the moving of the var
            // if self.left.present {
            //     println!("Left hand is holding: {}", self.left.what);
            // } else {
            //     println!("Left hand is holding nothing");
            // }

            show_item(&self.left, "Left"); // nested borrow here works
            show_item(&self.right, "Right");
        }
        pub fn juggle(mut self) -> Self {
            println!("Let's juggle");

            let air = self.left;
            self.left = self.right;
            self.right = air;
            self
        }

        pub fn new() -> Self {
            Self {
                left: Some(
                    // what: "an apple", doesn't work as "an apple" is borrowed
                    Fruit::Apple,
                ),
                right: Some(Fruit::Banana),
            }
        }
    }
}

use model::Hands;

pub fn about_ownership() {
    println!("\nOwnership is a concept of where the object exists in the ownership chain.");
    println!(
        "A string in its raw form is a constant string and exists for the lifetime of a program."
    );
    println!("A prop or var exists and is owned by another entity (&)");
    println!("A string with a capital s String is owned by the parent scope object e.g. Item");
    println!(
        "Lower case (with the &) is borrowed where you can access it but can't change its length."
    );

    let mut hands: Hands = Hands::new();

    hands.show_hands();
    println!("By not declaring a copying function, moving the var hands into the func will prevent it from being used");
    println!("We can manipulate the var by borrowing instead");
    hands = hands.juggle();
    hands.show_hands();
}
