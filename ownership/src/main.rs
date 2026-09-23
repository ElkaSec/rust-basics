fn main() {
    let s = String::from("alice");
    println!("before :{s}");

    let S = shout(s);

    println!("after :{S}")
}
fn shout(name: String) -> String {
    name.to_uppercase()
}

fn double(n: i32) -> i32 {
    n * 2
}
