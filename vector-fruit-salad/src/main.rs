/*
This program creates a fruit salad by scrambling (shuffling) a list of fruit.
A vector is a growable array. It can grow or shrink in size and is one of the most
useful data structures in Rust. A vector is represented using the Vec<T> type.
*/

use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::{Rng, thread_rng};
use clap::Parser;

#[derive(Parser, Debug)]

struct Args {
    fruits: Vec<String>
}

fn main() {

    let mut fruit = vec![
        "Orange",
        "Fig",
        "Pomegranate",
        "Cherry",
        "Apple",
        "Pear",
        "Peach",
    ];
    let args = Args::parse();

    if !args.fruits.is_empty() {
        args.fruits.iter().for_each(|f| fruit.push(f));
    }

    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    fruit.shuffle(&mut rng);

    let mut fruit_to_add = vec!["Banana", "Grapes", "Strawberry"];

    let num_to_add = rng.gen_range(1..=fruit_to_add.len());

    fruit_to_add.shuffle(&mut rng);

    for i in 0..num_to_add {
        fruit.push(fruit_to_add[i]);
    }

    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }

    }

    let random_fruit = fruit.choose(&mut rng).unwrap();   
    println!("random fruit: {}", random_fruit);
}
