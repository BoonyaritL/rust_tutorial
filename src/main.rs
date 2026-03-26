// use crate::basic::{functions, variables};

mod basic;
fn main() {
    basic::variables::run();
    // basic::variables::dbg();
    basic::variables::mut_variable();
    
     basic::functions::run();
     basic::strings::run();
}
