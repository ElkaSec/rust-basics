fn main() {
    for i in 1..=15 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
            continue;
        }
        if i % 3 == 0 {
            println!("Fizz");
            continue;
        }
        if i % 5 == 0 {
            println!("Buzz");
            continue;
        }
        println!("{}", i);
    }
}
