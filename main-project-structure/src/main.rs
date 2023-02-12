use crate::fizzbuzz::fizzbuzzer::fizzbuzzer;

mod fizzbuzz;
mod math;

fn main() {
    for n in 1..101 {
        println!("{:?}", fizzbuzzer(n))
    }
}
