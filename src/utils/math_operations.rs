pub fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// Computes the greatest common divisor (GCD) of `a` and `b` and returns a tuple
/// `(g, x, y)` where:
/// - `g` is the GCD of `a` and `b`,
/// - `x` and `y` are the coefficients satisfying the equation: `a * x + b * y = g`.
///
/// This function uses the Extended Euclidean Algorithm, which not only finds the GCD
/// but also the integers `x` and `y` that satisfy Bézout's identity.
///
/// # Parameters
/// - `a`: A non-negative integer.
/// - `b`: A non-negative integer.
///
/// # Returns
/// A tuple `(g, x, y)`:
/// - `g`: The GCD of `a` and `b`.
/// - `x`: An integer coefficient for `a` in Bézout's identity.
/// - `y`: An integer coefficient for `b` in Bézout's identity.
///
/// # Examples
/// ```
/// let (g, x, y) = extended_gcd(30, 20);
/// assert_eq!(g, 10); // GCD of 30 and 20 is 10
/// assert_eq!(30 * x + 20 * y, g); // Bézout's identity holds
/// ```
pub fn extended_gcd(a: u32, b: u32) -> (i32, i32, i32) {
    if a == 0 {
        return (b as i32, 0, 1);
    }
    let (g, x1, y1) = extended_gcd(b % a, a);
    let x = y1 - (b as i32 / a as i32) * x1;
    let y = x1;

    (g, x, y)
}


pub fn euler_phi(n: u32) -> u32 {
    let mut result = 1;
    for i in 2..n {
        if gcd(i, n) == 1 {
            result += 1;
        }
    }
    result
}

pub fn factorial(n: u32) -> u32 {
    let mut result = 1;
    for i in 1..n {
        result *= i;
    }
    result
}

pub fn mod_inverse(a: usize, m: usize) -> Option<usize> {
    for x in 1..m {
        if (a * x) % m == 1 {
            return Some(x);
        }
    }
    None
}

pub fn binary_decomposition(mut num: u64) -> Vec<u64> {
    let mut powers_of_two = Vec::new();
    let mut power = 0;

    while num > 0 {
        if num % 2 == 1 {
            powers_of_two.push(2u64.pow(power));
        }
        num /= 2;
        power += 1;
    }

    powers_of_two
}

pub fn find_modulus(powers_of_two: Vec<u64>, num: u64, d: u64) -> u64 {
    let max_num = *powers_of_two.iter().max().unwrap();
    let bit_num = 64 - max_num.leading_zeros();
    let mut vec_mod: Vec<u64> = vec![num];

    let mut current_val = num;
    for _ in 0..bit_num - 1 {
        let q = current_val.pow(2) % d;
        vec_mod.push(q);

        current_val = q;
    }


    let mut result = 1;
    for number in powers_of_two {
        let bit_num: usize = 63 - number.leading_zeros() as usize;
        result *= vec_mod[bit_num];
        result = result % d;
    }

    result
}