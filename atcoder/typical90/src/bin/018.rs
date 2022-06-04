use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::HashMap;
use std::f64::consts::PI;
use std::mem::swap;
const TAU: f64 = PI * 2.;

#[fastout]
fn main() {
    input! {
        T: f64,
        L: f64,
        x0: f64,
        y0: f64,
        Q: usize,
        E: [f64; Q],
    }

    for t in E {
        let phase = (-t / T * TAU - TAU / 4.);

        let x = 0.;
        let y = L / 2. * phase.cos();
        let z = L / 2. * phase.sin() + L / 2.;

        let height = z;
        let bottom = ((x - x0).powf(2.) + (y - y0).powf(2.)).sqrt();

        let res = (height / bottom).atan() * 360. / TAU;

        println!("{}", &res);
    }
}
