fn main() {
    let (a, b) = analyze([12.0, 8.5, 17.0, 14.5]);

    println!(" avg ={a}; max {b}")
}

fn analyze(grades: [f64; 4]) -> (f64, f64) {
    let mut max = grades[0];
    let mut acc: f64 = 0.0;

    for i in 0..4 {
        if grades[i] > max {
            max = grades[i];
        }
        acc += grades[i];
    }

    (acc / 4.00, max)
}
