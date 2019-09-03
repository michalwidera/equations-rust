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

#[allow(unused_variables)]
fn sum( array_a: &[i32] , delta_a : num_rational::Rational  , array_b: &[i32] , delta_b : num_rational::Rational  ) {

    let delta_c = std::cmp::min( delta_a , delta_b );

    println!( "da {} | db {} | dc {}",delta_a,delta_b,delta_c );

    for i in 0..10 {
        let idx = (num_rational::Ratio::from_integer(i)*delta_a/delta_c).floor() ;
        print!("{} {} {} :", i , idx, num_rational::Ratio::from_integer(i) *delta_a);
        if delta_c == delta_a {
            println!("[{}|{}]", array_a[i as usize] , array_b[*idx.numer() as usize]);
        }
        else {
            println!("[{}|{}]", array_a[*idx.numer() as usize] , array_b[i as usize]);
        }
    }
}

#[allow(dead_code)]
fn main() {

    let array_a: [i32; 30] = array_init::array_init(|i| i as i32);
    let array_b: [i32; 30] = array_init::array_init(|i| (i as i32)*(-1) as i32);

    let vector_a : Vec<i32> = (0..30).map(|x| x).collect();
    let vector_b : Vec<i32> = (0..30).map(|x| -x).collect();

    let complex_integer = num::complex::Complex::new(10, 20);
    let complex_float = num::complex::Complex::new(10.1, 20.1);

    println!("Complex integer: {}", complex_integer);
    println!("Complex float: {}", complex_float);

    let r1 = num_rational::Ratio::new( 1 , 2 );
    let r2 = num_rational::Ratio::new( 3 , 4 );

    println!("r1 = {}", r1);
    println!("r2 = {}", r2);
        
    let complex_rational = num::complex::Complex::new(r1, r2);
    println!("Complex rational: {}", complex_rational);

    print_vector( &vector_a );
    print_vector( &vector_b );

    print_array( & array_a );
    print_array( & array_b );

    println!("Floor 1/2 = {}" , r1.floor());
    println!("Floor 1/2 = {}" , r1.ceil());
    println!("Floor 4/3 = {}" , r2.floor());
    println!("Floor 4/3 = {}" , r2.ceil());

    sum( &array_a , r1 , &array_b , r2 );
}
