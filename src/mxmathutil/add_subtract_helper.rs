
pub mod add_mod {    
    use crate::mxmathutil::advancedmath::adv_math_helper::advmath_mod;

    pub fn add(a: i32, b: i32) -> i32 {        
        let c = a + b;
        return c;
    }
    
    pub fn special_addition(){
        advmath_mod::most_adv_addition();
        println!("special_addition called")
    }
}

pub mod subtract_mod {
    pub fn subtract(){
        println!("subtract called");
    }
}