#![feature(step_by)]

#[macro_use] extern crate itertools;
use itertools::Itertools;

fn main() {

    println!("\nBasic Range");
    for i in 1..11 {
        println!("i = {:3}; i*i = {:3}", i, i * i);
    }

    println!("\nRange with step. Notice this requires unstable \
        feature use #![feature(step_by)]) available in nightly only");
    for i in (0..11).step_by(2) {
        println!("i = {:2}", i);
    }

    println!("\nSame can be done with a filter (using closures). \
        Doesn't require unstable features and is way more flexible.");
    for i in (0..11).filter(|x| x % 2 == 0) {
        println!("i = {:3}", i);
    }

    println!("\nSame using itertools.");
    for i in (0..11).step(2) {
        println!("i = {:3}", i);
    }

    println!("\nReverse range");
    for i in (0..11).rev() {
        println!("i = {:2}", i);
    }

    println!("\nReverse range with a filter");
    for i in (-10..11).rev().filter(|x| x % 3 == 0) {
        println!("i = {:3}", i);
    }

    println!("\nExample of a consumer (collect). Produces a vector of 1 to 10.");
    let v = (1..11).collect::<Vec<i32>>();
    println!("v = {:?}", v);

    println!("\nIterate over the vector. Good Rustic style.");
    for n in v.iter() {
        println!("n = {:2}", n);
    }

    println!("\nC-style loop. This is considered bad form in Rust, \
        although can be useful if you want to yield not only vector's elements, \
        but their indexes as well.");
    for i in 0..v.len() {
        println!("v[{}] = {:2}", i, v[i]);
    }

    println!("\nMore ideomatic Rust is to use the enumerate adaptor on the iterator. \
        This yields tuples of (index, value)");
    for (i, n) in v.iter().enumerate() {
        println!("v[{}] = {:2}", i, n);
    }

    println!("\nUsing itertools to sort and dedup a vector");
    let data = vec![1, 4, 2, 3, 3, 2, 5, 1];
    println!("data = {:?}", data);
    let p = data
            .into_iter().sorted()
            .into_iter().dedup()
            .into_iter().collect::<Vec<i32>>();
    println!("data = {:?}", p);

    println!("\nUsing itertools to find min and max values of the vector");
    match p.iter().minmax().into_option() {
        None => println!("Could not find min and max values"),
        Some(minmax) => {
            let (min, max) = minmax;
            println!("min = {}, max = {}", min, max)
        },
    }
}

