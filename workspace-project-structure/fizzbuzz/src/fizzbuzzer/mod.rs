use math::arithmetic::modulo;
pub mod buzzer;

pub fn fizzbuzzer(x: i32) -> String {
    buzzer::buzzer();
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
