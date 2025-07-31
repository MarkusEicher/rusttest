fn main() {
    let mut input = [23, 82, 16, 45, 21, 94, 12, 34];

    input.sort();
    // println!("{:?}", input);

    let smallest = input[0];
    let largest = input[7];

    println!("{} is largest and {} is smallest", largest, smallest);
}
