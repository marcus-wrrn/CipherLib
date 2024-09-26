pub fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
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

pub fn mod_inverse(a: u8, m: u8) -> Option<u8> {
    for x in 1..m {
        if (a * x) % m == 1 {
            return Some(x);
        }
    }
    None
}