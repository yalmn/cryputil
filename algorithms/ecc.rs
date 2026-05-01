use crate::core::error::{CalcError, CalcResult};
use crate::core::math::{is_prime, mod_inv, rem_euclid};
use crate::core::trace::Trace;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Point {
    Infinity,
    Affine(i128, i128),
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Point::Infinity => write!(f, "O"),
            Point::Affine(x, y) => write!(f, "({}, {})", x, y),
        }
    }
}

// Input: a, b, p (prim)
// Calc:  Diskriminanten-Prüfung 4a^3 + 27b^2 ≠ 0 mod p
// Output: ok oder Fehler
fn check_curve(a: i128, b: i128, p: i128) -> CalcResult<()> {
    if !is_prime(p) {
        return Err(CalcError::NichtPrim(p.to_string()));
    }
    let disc = rem_euclid(4 * a * a * a + 27 * b * b, p);
    if disc == 0 {
        return Err(CalcError::UngueltigeEingabe(
            "Singuläre Kurve (Diskriminante = 0)".into(),
        ));
    }
    Ok(())
}

// Input: P, Q, a, p
// Calc:  Punktaddition auf E: y² = x³ + a x + b mod p
// Output: P + Q
pub fn add(p1: Point, p2: Point, a: i128, p: i128) -> CalcResult<Point> {
    match (p1, p2) {
        (Point::Infinity, q) => Ok(q),
        (q, Point::Infinity) => Ok(q),
        (Point::Affine(x1, y1), Point::Affine(x2, y2)) => {
            if x1 == x2 && rem_euclid(y1 + y2, p) == 0 {
                return Ok(Point::Infinity);
            }
            let lambda = if x1 == x2 && y1 == y2 {
                let num = rem_euclid(3 * x1 * x1 + a, p);
                let den = mod_inv(rem_euclid(2 * y1, p), p)?;
                rem_euclid(num * den, p)
            } else {
                let num = rem_euclid(y2 - y1, p);
                let den = mod_inv(rem_euclid(x2 - x1, p), p)?;
                rem_euclid(num * den, p)
            };
            let x3 = rem_euclid(lambda * lambda - x1 - x2, p);
            let y3 = rem_euclid(lambda * (x1 - x3) - y1, p);
            Ok(Point::Affine(x3, y3))
        }
    }
}

// Input: k, P, a, p
// Calc:  Skalarmultiplikation k·P per Double-and-Add
// Output: Punkt
pub fn scalar_mul(k: i128, p_pt: Point, a: i128, p: i128) -> CalcResult<Point> {
    if k < 0 {
        let neg = match p_pt {
            Point::Infinity => Point::Infinity,
            Point::Affine(x, y) => Point::Affine(x, rem_euclid(-y, p)),
        };
        return scalar_mul(-k, neg, a, p);
    }
    let mut result = Point::Infinity;
    let mut addend = p_pt;
    let mut n = k;
    while n > 0 {
        if n & 1 == 1 {
            result = add(result, addend, a, p)?;
        }
        addend = add(addend, addend, a, p)?;
        n >>= 1;
    }
    Ok(result)
}

// Input: a, b, p, P, Q
// Calc:  Trace einer Punktaddition
// Output: Trace
pub fn add_trace(a: i128, b: i128, p: i128, p1: Point, p2: Point) -> CalcResult<Trace> {
    check_curve(a, b, p)?;
    let mut t = Trace::new("Elliptische Kurve: Punktaddition");
    t.input("Kurve", format!("y² = x³ + {}x + {} mod {}", a, b, p));
    t.input("P", p1);
    t.input("Q", p2);
    let s = t.step("Addition");
    let r = add(p1, p2, a, p)?;
    t.line(s, format!("P + Q = {}", r));
    t.result("Ergebnis", r);
    Ok(t)
}

// Input: a, b, p, k, P
// Calc:  Trace einer Skalarmultiplikation
// Output: Trace
pub fn scalar_trace(a: i128, b: i128, p: i128, k: i128, p1: Point) -> CalcResult<Trace> {
    check_curve(a, b, p)?;
    let mut t = Trace::new("Elliptische Kurve: Skalarmultiplikation");
    t.input("Kurve", format!("y² = x³ + {}x + {} mod {}", a, b, p));
    t.input("k", k);
    t.input("P", p1);
    let s = t.step("Double-and-Add");
    let r = scalar_mul(k, p1, a, p)?;
    t.line(s, format!("k · P = {}", r));
    t.result("Ergebnis", r);
    Ok(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ec_add() {
        let r = add(Point::Affine(3, 10), Point::Affine(9, 7), 1, 23).unwrap();
        assert!(matches!(r, Point::Affine(_, _)));
    }
}
