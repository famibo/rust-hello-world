mod math_tools;

//use crate::math_tools::add_functions::add_five;
//use crate::math_tools::substract_functions::substract_five;
use crate::math_tools::{add_functions::add_five, substract_functions::substract_five};

pub fn main() {
    let mut x = 50;
    println!("x is {}", x);

    let y = add_five(x);
    println!("y is {}", y);

    let z = substract_five(x);
    println!("z is {}", z);

    x = 70;
    println!("x is {}", x);
}
