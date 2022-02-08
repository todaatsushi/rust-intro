mod model {
    pub enum Item<T> {
        Something(T),
        Nothing,
    }

    pub trait Displayable {
        fn display(&self) -> String;
    }

    enum Fruit {
        Apple,
        Banana,
        Kiwi,
    }

    impl Displayable for Fruit {
        fn display(&self) -> String {
            match self {
                Fruit::Apple => "an apple".to_owned(),
                Fruit::Banana => "a banana".to_owned(),
                Fruit::Kiwi => "a kiwi".to_owned(),
            }
        }
    }

    impl<T: Displayable> Item<T> {
        pub fn show_item(&self, which: &str) {
            match self {
                Item::Something(fruit) => {
                    println!("{} hand is holding {}", which, fruit.display())
                }
                _ => {
                    println!("{} hand is not holding anything", which)
                }
            }
        }
    }

    pub struct Hands {
        left: Item<Fruit>,
        right: Item<Fruit>,
    }

    impl Hands {
        pub fn show_hands(&self) {
            // We can borrow hands instead preventing the moving of the var
            // if self.left.present {
            //     println!("Left hand is holding: {}", self.left.what);
            // } else {
            //     println!("Left hand is holding nothing");
            // }

            Item::show_item(&self.left, "Left"); // nested borrow here works
            Item::show_item(&self.right, "Right");
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
                left: Item::Something(
                    // what: "an apple", doesn't work as "an apple" is borrowed
                    Fruit::Apple,
                ),
                right: Item::Something(Fruit::Banana),
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
