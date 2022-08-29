//this is private, 
//so it can be used only into current scope of basic_algebra_operations only
fn a_plus_b_whole_square(a: i32, b: i32) -> i32{ 
    return i32::pow(a +b, 2);
}

pub fn a_plus_b_whole_cube(a: i32, b: i32) -> i32{
    return i32::pow(a +b, 3);
}

pub mod basic_algebra_mod_v2{ //new mod

use crate::mxmathutil::algebra::basicalgebra::basic_algebra_operations;

    pub fn some_basic_algebra_fn2(){        
        println!("some_basic_algebra_fn2() called");
    }

    pub fn compute_a_plus_b_whole_square(a: i32, b: i32){
        let result = basic_algebra_operations::a_plus_b_whole_square(a, b);
        println!("whole square result: {}",result);
    }

}