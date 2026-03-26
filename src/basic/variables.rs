pub fn run() {
    let x = 10; // immutable variable
    let a:i32 = 5;
    // x+1;
    println!("a = {}", a);
    println!("x = {}", x);
}

// pub fn dbg() {
//     let a = 10;
//     let b = 20;
//     let c = dbg!(a + b);
//     println!("[src/main.rs.] a + b = {}", c);
// }

// mutable variable
pub fn mut_variable() {
    let mut x = 10;
    println!("x = {}", x);
    x = 20;
    println!("x mut = {}", x);
}