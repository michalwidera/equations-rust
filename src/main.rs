extern crate num;
extern crate num_rational;
extern crate array_init;

fn print_vector( input : &Vec<i32> ) {
    for data in input {
        print!(" {}", data);
    }
    println!("");
}

fn print_array( array : &[i32] ) {
    for data in array {
        print!(" {}", data);
    }
    println!("");
}

#[allow(dead_code)]
fn main() {

    let vector_a : Vec<i32> = (0..10).map(|x| x).collect();
    let vector_b : Vec<i32> = (0..10).map(|x| -x).collect();

    let array_a: [i32; 20] = array_init::array_init(|i| i as i32);
    let array_b: [i32; 20] = array_init::array_init(|i| (i as i32)*(-1) as i32);

    let complex_integer = num::complex::Complex::new(10, 20);
    let complex_float = num::complex::Complex::new(10.1, 20.1);

    println!("Complex integer: {}", complex_integer);
    println!("Complex float: {}", complex_float);
    let r1 = num_rational::Ratio::new( 1 , 2 );
    let r2 = num_rational::Ratio::new( 3 , 4 );
    
    let complex_rational = num::complex::Complex::new(r1, r2);
    println!("Complex rational: {}", complex_rational);

    print_vector( &vector_a );
    print_vector( &vector_b );

    print_array( & array_a );
    print_array( & array_b );
}
