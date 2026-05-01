use crate::core::error::{CalcError, CalcResult};
use crate::core::math::rem_euclid;
use crate::core::trace::Trace;

// Input: a, n (n > 0)
// Calc:  additives Inverses (-a) mod n
// Output: Trace
pub fn additive_inverse(a: i128, n: i128) -> CalcResult<Trace> {
    if n <= 0 {
        return Err(CalcError::Bereich("n muss > 0 sein".into()));
    }
    let mut t = Trace::new("Additives Inverses modulo n");
    t.input("a", a);
    t.input("n", n);
    let s = t.step("Berechnung von -a mod n");
    let ar = rem_euclid(a, n);
    t.line(s, format!("a mod n = {}", ar));
    let inv = rem_euclid(-ar, n);
    t.line(s, format!("-a mod n = (n - a) mod n = {}", inv));
    let probe = rem_euclid(ar + inv, n);
    t.line(s, format!("Prüfung: a + (-a) mod n = {}", probe));
    t.result("Additives Inverses", inv);
    Ok(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_additive_inverse() {
        let t = additive_inverse(3, 7).unwrap();
        assert_eq!(t.result[0].1, "4");
    }
}
