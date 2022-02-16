use std::collections::{BTreeMap, HashMap};

pub fn about_hashmaps() {
    println!("\nAbout hashmaps which are rust versions of dicts / hash tables");
    let mut grid = HashMap::new();
    grid.insert((2, 3), "tree");
    grid.insert((4, 7), "rock");

    for (key, value) in &grid {
        println!("{} at {:?}", value, key);
    }

    println!("Fetch val by {:?}", grid.entry((2, 3)));
    println!("Can also set a default");
    grid.entry((3, 19)).or_insert("Bird");
    println!("val {:?}", grid.entry((3, 19)));

    println!("Or insert returns a mutable reference so we can assign it");
    *grid.entry((1, 4)).or_insert("") = "YouTube"; // Have to dereference as we're getting a mutable reference
    println!("val {:?}", grid.entry((1, 4)));

    println!("To remove we pass a borrowed key (as we need to borrow to remove)");
    grid.remove(&(1, 4));

    println!("We can also index: {}", grid[&(3, 19)]);
    println!(
        "Program panics when the key doesn't exist, we need to use the `get` method to avoid it"
    );
    // println!("{}", grid[&(4, 59)]); // Nothing there

    if let Some(cell) = grid.get(&(22, 111)) {
        println!("Shouldn't print {}", cell);
    } else {
        println!("Nothing found");
    }

    println!("Keys must be hashable!");
    println!(
        "\n We can use a b tree map as an alternative where most of the functionality is the same"
    );
    println!("Keys must have a sense of order");
    let mut b_grid = BTreeMap::new();
    b_grid.insert((2, 3), "tree");
    b_grid.insert((4, 7), "rock");
    for (key, value) in &b_grid {
        println!("{} at {:?}", value, key);
    }
    b_grid.entry((3, 19)).or_insert("Bird");
    *b_grid.entry((1, 4)).or_insert("") = "YouTube"; // Have to dereference as we're getting a mutable reference
    b_grid.remove(&(1, 4));
    // println!("{}", grid[&(4, 59)]); // Nothing there
    if let Some(cell) = b_grid.get(&(22, 111)) {
        println!("Shouldn't print {}", cell);
    } else {
        println!("Nothing found");
    }
}
