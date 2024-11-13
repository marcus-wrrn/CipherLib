pub fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

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