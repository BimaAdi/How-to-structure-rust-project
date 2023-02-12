use crate::math::arithmetic::modulo;

use crate::fizzbuzz::buzzer::buzzer;

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
