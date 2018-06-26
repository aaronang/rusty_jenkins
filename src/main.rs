extern crate rand;

use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Hello, world!");
    println!("Secrent number: {}!", secret_number);
    println!("Bye, world!");
}
