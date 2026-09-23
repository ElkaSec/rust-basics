fn main() {
    let (a, b) = analyze([12.0, 8.5, 17.0, 14.5]);

    println!("average : {}, max : {}", a, b);
}

fn analyze(grades: [f64; 4]) -> (f64, f64) {
    let mut acc: f64 = 0.00;
    let mut max = grades[0];

    for i in 0..=3 {
        if max <= grades[i] {
            max = grades[i];
        }
        acc += grades[i];
    }
    let avg = acc / 4.00;

    return (avg, max);
}
