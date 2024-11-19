//use crate::utils::math_operations::mod_inverse;

use crate::utils::math_operations::mod_inverse;

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

pub struct EllipticCurve {
    a: i64,    
    group_num: i64,
}

impl EllipticCurve {
    pub fn new(a: i64, group_num: i64) -> Self {
        if group_num < 0 {
            panic!("Group number cannot be negative");
        }

        EllipticCurve {
            a,
            group_num
        }
    }

    pub fn group_number(&self) -> usize {
        self.group_num as usize
    }

    pub fn addition(&self, p: Point, q: Point) -> Option<Point> {
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

    pub fn doubling(&self, p: Point) -> Option<Point> {
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
}

/// Method for generating groups taking a generator point p and a curve
/// Returns a group up to the point at infinity
pub fn generate_group(curve: EllipticCurve, p: Point) -> Vec<Point> {
    let mut group_vals: Vec<Point> = vec![];

    let mut q = p;
    loop {
        group_vals.push(q);

        let next_q = if p == q {
            curve.doubling(p)
        } else {
            curve.addition(p, q)
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