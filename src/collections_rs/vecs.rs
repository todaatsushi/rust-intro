use std::collections::VecDeque;

pub fn about_vecs() {
    println!("First, about vectors which stores lists of things");
    let mut primes: Vec<i32> = Vec::new();

    println!("We can add to vecs if they're the same type e.g. this vec of prime numbers");
    primes.push(1);
    primes.push(2);
    primes.push(3);
    println!("{:?}", primes);

    println!("We can initialise vecs with a shorthand macro.");
    primes = vec![1, 2, 3];
    println!("{:?}", primes);

    println!("Similar methods exist to remove");
    primes.remove(2);
    println!("{:?}", primes);

    println!("And we can index.");
    println!("Num at index 0 is {}", primes[0]);

    println!("\nSimple loop using indicies to show primes");
    let mut i = 0;
    while i < primes.len() {
        println!("Num at index {} is {}", i, primes[i]);
        i += 1;
    }

    println!("For every ds in a collection, we can iterate easier using iter");
    let mut primes_iter = primes.iter();
    while let Some(prime) = primes_iter.next() {
        println!("Prime is {}", prime);
    }

    println!("We can also enumerate.");
    let mut primes_iter_enumerate = primes.iter().enumerate();
    while let Some((index, num)) = primes_iter_enumerate.next() {
        println!("Num at index {} is {}", index, num);
    }

    println!("Refactored to use a for loop");
    for (index, num) in primes.iter().enumerate() {
        println!("Num at index {} is {}", index, num);
    }

    println!("\nVec that is more efficient to change just the ends of the vec is a vec dequeue.");
    let mut primes_dequeue = VecDeque::new();
    primes_dequeue.push_front(2);
    primes_dequeue.push_back(3);
    primes_dequeue.push_front(1);
    println!("{:?}", primes_dequeue);
}
