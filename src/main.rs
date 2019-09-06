use num::complex::Complex;
use num_rational::Ratio;
use num_rational::Rational;

// summary of two time series and const delta
fn sum(vec_a: &[isize], del_a: Rational, vec_b: &[isize], del_b: Rational) -> Vec<(isize, isize)> {
    let mut result = Vec::<(isize, isize)>::new();

    let z = std::cmp::min(del_a, del_b);

    for i in 0..10 {
        if z == del_a {
            let idx = (Ratio::from_integer(i) * del_a / del_b).floor();
            result.push((vec_a[i as usize], vec_b[*idx.numer() as usize]));
        } else {
            let idx = (Ratio::from_integer(i) * del_b / del_a).floor();
            result.push((vec_a[*idx.numer() as usize], vec_b[i as usize]));
        }
    }

    result
}

// hash function on two time series and const delta
fn hash(vec_a: &[isize], del_a: Rational, vec_b: &[isize], del_b: Rational) -> Vec<isize> {
    let mut result = Vec::<isize>::new();

    let z = del_b / (del_a + del_b);

    for i in 0..20 {
        let idx = Ratio::from_integer(i);
        if (z * idx).floor() == ((idx + 1) * z).floor() {
            result.push(vec_b[*(idx - ((idx + 1) * z).floor()).numer() as usize]);
        } else {
            result.push(vec_a[*((idx * z).floor()).numer() as usize]);
        }
    }

    result
}

// testing on accelerated delta
fn hash_test(vec_a: &[isize], vec_b: &[isize]) -> Vec<isize> {
    fn del_a(n: isize) -> Rational {
        Ratio::new(1, 2) + Ratio::new(n, 20)
    }

    fn del_b(n: isize) -> Rational {
        Ratio::new(1, 1) + Ratio::new(n, 10)
    }

    fn z(n: isize) -> Rational {
        del_b(n) / (del_a(n) + del_b(n))
    }

    let mut result = Vec::<isize>::new();

    for i in 0..20 {
        let idx = Ratio::from_integer(i);
        if (z(i) * idx).floor() == ((idx + 1) * z(i + 1)).floor() {
            result.push(vec_b[*(idx - ((idx + 1) * z(i)).floor()).numer() as usize]);
        } else {
            result.push(vec_a[*((idx * z(i)).floor()).numer() as usize]);
        }
    }

    result
}

fn main() {
    let vector_a: Vec<isize> = (1..30).map(|x| x).collect();
    let vector_b: Vec<isize> = (1..30).map(|x| -x).collect();

    let delta_a = Ratio::new(1, 2);
    let delta_b = Ratio::new(1, 1);

    println!("delta_a = {}", delta_a);
    println!("delta_b = {}", delta_b);

    let complex_rational = Complex::new(delta_a, delta_b);
    println!("Complex rational: {}", complex_rational);

    println!("vector A {:?}", &vector_a);
    println!("vector B {:?}", &vector_b);

    println!("Floor 1/2 = {}", delta_a.floor());
    println!("Ceil  1/2 = {}", delta_a.ceil());

    let v_result = sum(&vector_a, delta_a, &vector_b, delta_b);
    println!("sum: {:?}", v_result);

    let v_result = hash(&vector_a, delta_a, &vector_b, delta_b);
    println!("hash: {:?}", v_result);

    let v_result = hash_test(&vector_a, &vector_b);
    println!("hash_test: {:?}", v_result);
}
