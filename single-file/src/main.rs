// Move to folder math -> modulo
fn modulo(x: i32, y: i32) -> i32 {
    x % y
}

// Move to folder fizzbuzz -> fizzbuzzer -> buzzer
fn buzzer() {
    println!("bzzzzzzz!!!")
}

// Move to folder fizzbuzz -> fizzbuzzer
fn fizzbuzzer(x: i32) -> String {
    buzzer();
    if modulo(x, 3) == 0 && modulo(x, 5) == 0 {
        "fizzbuzz".to_owned()
    } else if modulo(x, 3) == 0 {
        "fizz".to_owned()
    } else if modulo(x, 5) == 0 {
        "buzz".to_owned()
    } else {
        x.to_string()
    }
}

fn main() {
    for n in 1..101 {
        println!("{:?}", fizzbuzzer(n))
    }
}
