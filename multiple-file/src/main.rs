pub mod fizzbuzz;
pub mod math;
use fizzbuzz::fizzbuzzer;

fn main() {
    for n in 1..101 {
        println!("{:?}", fizzbuzzer(n))
    }
}
