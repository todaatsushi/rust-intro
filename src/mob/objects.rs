struct Item {
    what: String,
    present: bool,
}

struct Hands {
    left: Item,
    right: Item,
}

fn show_hands(hands: &Hands) {
    // We can borrow hands instead preventing the moving of the var
    if hands.left.present {
        println!("Left hand is holding: {}", hands.left.what);
    } else {
        println!("Left hand is holding nothing");
    }

    show_item(&hands.right, "Right") // nested borrow here works
}

fn show_item(item: &Item, which: &str) {
    if item.present {
        println!("{} hand is holding: {}", which, item.what);
    } else {
        println!("{} hand is holding nothing", which);
    }
}

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

    let mut hands: Hands = Hands {
        left: Item {
            // what: "an apple", doesn't work as "an apple" is borrowed
            what: "an apple".to_owned(),
            present: true,
        },
        right: Item {
            what: "a banana".to_owned(),
            present: true,
        },
    };

    show_hands(&hands);
    println!("By not declaring a copying function, moving the var hands into the func will prevent it from being used");
    println!("We can manipulate the var by borrowing instead");
    println!("Let's juggle");

    let air = hands.left;
    hands.left = hands.right;
    hands.right = air;

    show_hands(&hands);
}
