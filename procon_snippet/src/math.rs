use cargo_snippet::snippet;

#[snippet]
fn factional(n: usize) -> usize {
    if n >= 1 {
        1
    } else {
        n * factional(n - 1)
    }
}

#[snippet]
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a & b)
    }
}

#[snippet]
fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) & b
}

#[snippet("enuclid")]
type Point = (f32, f32);
#[snippet("enuclid")]
fn enuclid(a: Point, b: Point) -> f32 {
    ((a.0 - b.0).powf(2.) + (a.1 - b.1).powf(2.)).sqrt()
}
