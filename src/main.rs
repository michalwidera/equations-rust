use num::complex::Complex;
use num_rational::Ratio;
use num_rational::Rational;

fn sum(
    vector_a: &Vec<i32>,
    delta_a: Rational,
    vector_b: &Vec<i32>,
    delta_b: Rational,
) -> Vec<(i32, i32)> {
    let mut result = Vec::<(i32, i32)>::new();

    let delta_c = std::cmp::min(delta_a, delta_b);

    for i in 0..10 {
        if delta_c == delta_a {
            let idx = (Ratio::from_integer(i) * delta_a / delta_b).floor();
            result.push((vector_a[i as usize], vector_b[*idx.numer() as usize]));
        } else {
            let idx = (Ratio::from_integer(i) * delta_b / delta_a).floor();
            result.push((vector_a[*idx.numer() as usize], vector_b[i as usize]));
        }
    }

    result
}

fn hash(
    vector_a: &Vec<i32>,
    delta_a: Rational,
    vector_b: &Vec<i32>,
    delta_b: Rational,
) -> Vec<i32> {
    let mut result = Vec::<i32>::new();

    let delta = delta_b / (delta_a + delta_b);

    for i in 0..20 {
        let idx = Ratio::from_integer(i);
        if (delta * idx).floor() == ((idx + 1) * delta).floor() {
            result.push(vector_b[*(idx - ((idx + 1) * delta).floor()).numer() as usize]);
        } else {
            result.push(vector_a[*((idx * delta).floor()).numer() as usize]);
        }
    }

    result
}

fn main() {
    let vector_a: Vec<i32> = (0..30).map(|x| x).collect();
    let vector_b: Vec<i32> = (0..30).map(|x| -x).collect();

    let delta_a = Ratio::new(1, 2);
    let delta_b = Ratio::new(1, 1);

    println!("delta_a = {}", delta_a);
    println!("delta_b = {}", delta_b);

    let complex_rational = Complex::new(delta_a, delta_b);
    println!("Complex rational: {}", complex_rational);

    println!("vector A {:?}", &vector_a);
    println!("vector B {:?}", &vector_b);

    println!("Floor 1/2 = {}", delta_a.floor());
    println!("Floor 1/2 = {}", delta_a.ceil());

    let v_result = sum(&vector_a, delta_a, &vector_b, delta_b);
    println!("sum: {:?}", v_result);

    let v_result = hash(&vector_a, delta_a, &vector_b, delta_b);
    println!("hash: {:?}", v_result);
}
