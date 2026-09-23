fn main() {
    let s = String::from("alice");
    println!("{s}");
    let s = shout(s);
    println!("{s}");

    let n = 7;
    let n_d = double(n);
    println!("{n} doubled = {n_d}");
}

fn shout(name: String) -> String {
    name.to_uppercase()
}

fn double(n: i32) -> i32 {
    n * 2
}
