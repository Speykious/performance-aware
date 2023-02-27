fn main() {
    println!("{{ \"pairs\": [");
    let (start, end) = (-100.0, 100.0);
    for _ in 0..10_000_000 {
        let (x0, y0) = rand_point(start, end);
        let (x1, y1) = rand_point(start, end);
        println!("  {{ \"x0\": {x0}, \"y0\": {y0}, \"x1\": {x1}, \"y1\": {y1} }},");
    }
    println!("] }}");
}

fn rand_point(start: f64, end: f64) -> (f64, f64) {
    let (a, b) = rand::random::<(f64, f64)>();
    (start + a * (end - start), start + b * (end - start))
}
