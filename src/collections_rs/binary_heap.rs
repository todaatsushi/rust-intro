use std::collections::BinaryHeap;

pub fn about_binary_heaps() {
    println!("\nThe binary heap");
    let mut primes = BinaryHeap::new();
    primes.push(1);
    primes.push(2);
    primes.push(3);

    println!("The binary heap doesn't store the values in a particular order when iterated upon.");
    for prime in &primes {
        println!("Num: {}", prime)
    }

    println!("As a heap, we access the values via pop which gets them as we expect");
    while let Some(prime) = primes.pop() {
        println!("Num: {}", prime)
    }
}
