// use std::backtrace;

fn main() {
    let mut x = 0;
    loop {
        if x < 5 {
            println!("x: {}", x);
            x += 1;
        } else {
            break;
        }
    }
    // let a = 1;
    // let b = 10;
    println!("Sum is: {}", add(1, 10))
}
// fn main() {
//     let i = 10;
//     if i > 5 {
//         let j = 20;
//         println!("i = {}, j = {}", i, j);
//     }
//     println!("i = {}", i);
// }

fn add(a: i32, b: i32) -> i32 {
    a + b
}