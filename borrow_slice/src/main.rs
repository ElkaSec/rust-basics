fn main() {
    let lang = String::from("rust systems");
    let longer = longer_len(&lang, "hello world");
    println!("longer:{longer}");

    let rust = &lang[..4];
    let systems = &lang[5..];
    println!("{rust} / {systems}");
    println!("{lang}");
}

fn longer_len(a: &str, b: &str) -> usize {
    let s = a.len();
    let t = b.len();
    if s > t { s } else { t }
}
