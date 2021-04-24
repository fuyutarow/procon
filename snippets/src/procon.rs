use cargo_snippet::snippet;

#[snippet(name = "procon")]
#[snippet(prefix = "use proconio::marker::Chars;")]
#[snippet(prefix = "use proconio::{fastout, input};")]
#[snippet(prefix = "")]
#[snippet(prefix = "#[fastout]")]
fn main() {
    todo!();
}

#[snippet("print1")]
println!("{}",);

#[snippet("printvec")]
println!(
    "{} ",
    v.iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join(" ")
);
