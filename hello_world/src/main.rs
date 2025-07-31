// fn main() {
//     println!("Hello, world! fib(6) {}", fib(6));
// }

// fn fib(n: u64) -> u64 {
//     if n <= 1 {
//         n
//     } else {
//         fib(n - 1) + fib(n - 2)
//     }
// }

// fn main() {
//     let mut some_x: i32 = 5;
//     println!("some_x = {}", some_x);
//     some_x = 6;
//     println!("some_x = {}", some_x);

//     let div = 2.4 / 2.0;
//     let int_div = 10 / 5;

//     println!("div = {}, int_div = {}", div, int_div);
// }

// fn main() {
//     let c = 'z';
//     let z = 'Z';
//     let emoji = "😊";
    
//     let s1: String = String::from("This is a string. 💎");

//     println!("c = {}, z = {}, emoji = {}", c, z, emoji);
//     println!("{}", s1);

// }

fn main() {
    let tup = (1, 2.0, 'A');
    println!("{:?}", tup);
    let (a, b, c) = tup;
    println!("({}, {}, {})", a, b, c);
    let another_tuple = (true, 42);
    println!("{}", another_tuple.1);

    let arr = ['A', 'B', 'C'];
    println!("{}", arr[0]);
    let [a, b, c] = arr;
    println!("[{}, {}, {}]", a, b, c)
}