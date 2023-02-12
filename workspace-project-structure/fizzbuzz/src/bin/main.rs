use fizzbuzz::fizzbuzzer::fizzbuzzer;

fn main() {
    for n in 1..101 {
        println!("{:?}", fizzbuzzer(n))
    }
}
