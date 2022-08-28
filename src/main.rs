mod mxmathutil;

use mxmathutil::add_subtract_helper::add_mod;
use mxmathutil::add_subtract_helper::subtract_mod;
use mxmathutil::multiplication_division_helper::multiplication_division_mod;

fn main() {
    let x: i32 = 30;
    let y: i32 = 2;
    println!("{} days", x + y);

    println!("let's go with rust.. ya");
    let result = add_mod::add(3, 5);
    println!("result is = {}", result);

    subtract_mod::subtract();

    multiplication_division_mod::multiply();
    multiplication_division_mod::division();

    add_mod::special_addition();

    crate::mxmathutil::advancedmath::adv_math_helper::advmath_mod::most_adv_addition();
}
