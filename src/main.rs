mod mxmathutil;

use mxmathutil::simplearithmetic::add_subtract_helper::add_mod;
use mxmathutil::simplearithmetic::add_subtract_helper::subtract_mod;
use mxmathutil::simplearithmetic::multiplication_division_helper::multiplication_division_mod;
use mxmathutil::algebra::factorials::factorial_operations;
use mxmathutil::algebra::basicalgebra::basic_algebra_operations::basic_algebra_mod_v2;

fn main() {    
    let result = add_mod::add(3, 5);
    println!("result is = {}", result);

    subtract_mod::subtract();

    multiplication_division_mod::multiply();
    multiplication_division_mod::division();

    add_mod::special_addition();

    //calling func with fully qualified module path
    mxmathutil::advancedarithmetic::adv_arithmetic_helper::advmath_mod::most_adv_addition();

    factorial_operations::print_factorial();

    basic_algebra_mod_v2::some_basic_algebra_fn2();
    basic_algebra_mod_v2::compute_a_plus_b_whole_square(2,7);

}
