use std::collections::{BTreeSet, HashSet};

pub fn about_hashsets() {
    println!("\nHashable values with no keys live inside a hash set.");
    let mut primes: HashSet<i32> = HashSet::new();

    primes.insert(1);
    primes.insert(3);
    primes.insert(2);
    println!("The primes again {:?}", primes);

    println!("\nNon-hashable values with no keys live inside a btree set.");
    let mut b_primes: BTreeSet<i32> = BTreeSet::new();

    b_primes.insert(1);
    b_primes.insert(3);
    b_primes.insert(2);
    println!("The primes again {:?} (But in order)", b_primes);
}
