use matrix::Matrix;
use num_complex::Complex;

fn main() {
    let a = Matrix::new(2, 1, Complex<f64>::new(1.0, 1.0));
    let b = Matrix::new(3, 3, 2);
    let c = a.tensor(&b);
    println!("{:?}", c);
    let d = a.clone();
}