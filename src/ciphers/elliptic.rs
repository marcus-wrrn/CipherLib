use crate::utils::math_operations::{mod_inverse, modpow};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: i64,
    pub y: i64
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Point {
            x,
            y
        }
    }
}

#[derive(Default)]
pub struct EllipticCurve {
    a: i64,    
    group_num: i64,
    order_num: usize,
    pub group: Vec<Point>
}

impl EllipticCurve {
    pub fn new(a: i64, group_num: i64, generator_p: &Point) -> Self {
        if group_num < 0 {
            panic!("Group number cannot be negative");
        }

        let mut curve = EllipticCurve {
            a,
            group_num,
            ..Default::default()
        };

        let group = generate_group(&curve, *generator_p);

        curve.order_num = group.len();
        curve.group = group;
        

        curve
    }

    pub fn addition(&self, p: &Point, q: &Point) -> Option<Point> {
        let numerator = (q.y - p.y).rem_euclid(self.group_num);
        let denominator = (q.x - p.x).rem_euclid(self.group_num);

        let Some(inv_denomiator) = mod_inverse(denominator as usize, self.group_num as usize) else {
            return None;
        };

        let slope = (inv_denomiator as i64 * numerator).rem_euclid(self.group_num);

        let x3 = (slope.pow(2) - p.x - q.x).rem_euclid(self.group_num);
        let y3 = (slope * (p.x - x3) - p.y).rem_euclid(self.group_num);

        return Some(Point::new(x3, y3));
    }

    pub fn doubling(&self, p: &Point) -> Option<Point> {
        let numerator = (3 * p.x.pow(2) + self.a).rem_euclid(self.group_num);
        let denominator = (2 * p.y).rem_euclid(self.group_num);

        let Some(inv_denomiator) = mod_inverse(denominator as usize, self.group_num as usize) else {
            return None;
        };

        let slope = (inv_denomiator as i64 * numerator).rem_euclid(self.group_num);

        let x3 = (slope.pow(2) - 2 * p.x).rem_euclid(self.group_num);
        let y3 = (slope * (p.x - x3) - p.y).rem_euclid(self.group_num);

        return Some(Point::new(x3, y3));
    }

    pub fn print_group(&self) {
        println!("Order: {}", self.group.len());
        for i in 0..self.group.len() {
            println!("Point {}: {:?}", i + 1, self.group[i]);
        }
    }
}

/// Method for generating groups taking a generator point p and a curve
/// Returns a group up to the point at infinity
pub fn generate_group(curve: &EllipticCurve, p: Point) -> Vec<Point> {
    let mut group_vals: Vec<Point> = vec![];

    let mut q = p;
    loop {
        group_vals.push(q);

        let next_q = if p == q {
            curve.doubling(&p)
        } else {
            curve.addition(&p, &q)
        };

        // If next point is defined set it to q else break out of the loop
        if let Some(next_point) = next_q {
            q = next_point;
        } else {
            break;
        }
    }

    group_vals
}

fn is_quad_residue(n: u32, p: u32) -> bool {
    modpow(n as u64, (p as u64 - 1)/2 , p as u64) == 1
}

fn modular_sqrt(n: u32, p: u32) -> Option<u32> {
    (0..p).find(|&y| (y * y) % p == n)
}

pub fn calc_points(p: u32, a: u32, b: u32) -> Vec<Point> {
    let mut points = Vec::new();

    for x in 0..p {
        
        let fx = (x.pow(3) + a * x + b) % p;
        
        if is_quad_residue(fx, p) {
            if let Some(y1) = modular_sqrt(fx, p) {
                points.push(Point::new(x as i64, y1 as i64));

                let y2 = (-(y1 as i64)).rem_euclid(p as i64);
                if y1 as i64 != y2 {
                    points.push(Point::new(x as i64, y2));
                }
            }
        }
    }

    points
}