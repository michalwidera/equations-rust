extern crate num;
extern crate num_rational;


#[allow(dead_code)]
fn main() {
    let complex_integer = num::complex::Complex::new(10, 20);
    let complex_float = num::complex::Complex::new(10.1, 20.1);

    println!("Complex integer: {}", complex_integer);
    println!("Complex float: {}", complex_float);
    let r1 = num_rational::Ratio::new( 1 , 2 );
    let r2 = num_rational::Ratio::new( 3 , 4 );
    
    let complex_rational = num::complex::Complex::new(r1, r2);
    println!("Complex rational: {}", complex_rational);
}
