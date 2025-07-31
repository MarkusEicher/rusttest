fn main() {

    let a = 30;
    let b = 50;
    if bigger(a, b) {
        println!("{} is bigger than {}", a, b);
    } else {
        println!("{} is smaller than {}", a, b);
    }
}

fn bigger(a: i32, b: i32) -> bool {
    // TODO
    if a > b {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_biggers() {
        assert!(bigger(20, 10));
        assert!(!bigger(10, 20));
    }
}
