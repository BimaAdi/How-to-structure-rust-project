use crate::fizzbuzz::buzzer::buzzer;
use crate::math::arithmetic::modulo;

pub fn fizzbuzzer(x: i32) -> String {
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
