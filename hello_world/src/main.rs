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

// fn main() {
//     let tup = (1, 2.0, 'A');
//     println!("{:?}", tup);
//     let (a, b, c) = tup;
//     println!("({}, {}, {})", a, b, c);
//     let another_tuple = (true, 42);
//     println!("{}", another_tuple.1);

//     let arr = ['A', 'B', 'C'];
//     println!("{}", arr[0]);
//     let [a, b, c] = arr;
//     println!("[{}, {}, {}]", a, b, c)
// }

// fn main() {
//     let mut s = String::from("Hello");
//     change(&mut s);
//     println!("{}", s);
// }

// // fn change(some_string: &String) { // This is an immutable ref
// fn change(some_string: &mut String) { // This is a mutable ref
//     some_string.push_str(", World!");
// }


// fn give_me_a_ref(input: &(String, i32)) -> &String {
//     &input.0
// }



struct ControlPoint {
    x: f64,
    y: f64,
    enabled: bool,
}
// #[derive(Debug)]
enum IpAddressType {
    Ipv4 (u8, u8, u8, u8),
    Ipv6 (u16, u16, u16, u16, u16, u16, u16, u16),
}

fn main () {
    let cp = ControlPoint {
        x: 10.5,
        y: 12.8,
        enabled: true,
    };
    println!("{}, {}, {}", cp.x, cp.y, cp.enabled);

    // let ip_type = IpAddressType::Ipv4;
    // print!("The ip address typ is: {:?}.", ip_type);

    // let _ipv4_home = IpAddressType::Ipv4(127, 0, 0, 1);
    // let _ipv6_home = IpAddressType::Ipv6(0, 0, 0, 0, 0, 0, 0, 1);
    let home = IpAddressType::Ipv4(127, 0, 0, 1);
    let loopback = IpAddressType::Ipv6(0, 0, 0, 0, 0, 0, 0, 1);

    match home {
        IpAddressType::Ipv4(a, b, c, d) => {
            println!("IPv4: {}.{}.{}.{}", a, b, c, d);
        }
        IpAddressType::Ipv6(..) => {
            println!("IPv6 address");
        }
    }

    match loopback {
        IpAddressType::Ipv6(a, b, c, d, e, f, g, h) => {
            println!("IPv6: {:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}", a, b, c, d, e, f, g, h);
        }
        IpAddressType::Ipv4(..) => {
            println!("IPv4 address");
        }
    }
}
