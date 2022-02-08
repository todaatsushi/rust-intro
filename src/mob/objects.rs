mod model {
    pub struct Item {
        what: String,
        present: bool,
    }

    impl Item {
        pub fn show_item(item: &Item, which: &str) {
            if item.present {
                println!("{} hand is holding: {}", which, item.what);
            } else {
                println!("{} hand is holding nothing", which);
            }
        }
    }

    pub struct Hands {
        left: Item,
        right: Item,
    }

    impl Hands {
        pub fn show_hands(&self) {
            // We can borrow hands instead preventing the moving of the var
            if self.left.present {
                println!("Left hand is holding: {}", self.left.what);
            } else {
                println!("Left hand is holding nothing");
            }

            Item::show_item(&self.right, "Right") // nested borrow here works
        }
        pub fn juggle(mut self) -> Self {
            println!("Let's juggle");

            let air: Item = self.left;
            self.left = self.right;
            self.right = air;
            self
        }

        pub fn new() -> Self {
            Self {
                left: Item {
                    // what: "an apple", doesn't work as "an apple" is borrowed
                    what: "an apple".to_owned(),
                    present: true,
                },
                right: Item {
                    what: "a banana".to_owned(),
                    present: true,
                },
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
