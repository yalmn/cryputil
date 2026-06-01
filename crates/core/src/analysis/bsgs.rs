use std::collections::HashMap;

use crate::core::error::{CalcError, CalcResult};
use crate::core::math::{is_prime, isqrt, mod_pow, rem_euclid};
use crate::core::trace::{Table, Trace};

// Input: g, h, p (prim)
// Calc:  Baby-Step-Giant-Step zur Berechnung von x mit g^x ≡ h mod p
// Output: Trace
pub fn solve(g: i128, h: i128, p: i128) -> CalcResult<Trace> {
    if !is_prime(p) {
        return Err(CalcError::NichtPrim(p.to_string()));
    }
    let mut t = Trace::new("Baby-Step-Giant-Step");
    t.input("g", g);
    t.input("h", h);
    t.input("p", p);

    let s1 = t.step("Suchgrenze berechnen");
    let m = isqrt(p - 1) + if isqrt(p - 1).pow(2) == p - 1 { 0 } else { 1 };
    t.line(s1, format!("m = ceil(sqrt(p - 1)) = {}", m));

    let s2 = t.step("Baby-Steps berechnen");
    let mut baby: HashMap<i128, i128> = HashMap::new();
    let headers = vec!["j".into(), "g^j mod p".into()];
    let mut rows = Vec::new();
    let mut acc: i128 = 1;
    for j in 0..m {
        baby.entry(acc).or_insert(j);
        rows.push(vec![j.to_string(), acc.to_string()]);
        acc = rem_euclid(acc * g, p);
    }
    t.table(s2, Table { headers, rows });

    let s3 = t.step("Giant-Steps und Suche");
    let factor = mod_pow(g, m * (p - 2), p)?;
    t.line(
        s3,
        format!(
            "g^(-m) ≡ g^(m·(p-2)) mod p = {}^({}·{}) mod {} = {}",
            g,
            m,
            p - 2,
            p,
            factor
        ),
    );
    let headers = vec!["i".into(), "h · (g^-m)^i mod p".into(), "Match j".into()];
    let mut rows = Vec::new();
    let mut gamma = rem_euclid(h, p);
    for i in 0..m {
        let m_str = baby
            .get(&gamma)
            .map(|j| j.to_string())
            .unwrap_or("-".into());
        rows.push(vec![i.to_string(), gamma.to_string(), m_str]);
        if let Some(&j) = baby.get(&gamma) {
            let x = rem_euclid(i * m + j, p - 1);
            t.table(s3, Table { headers, rows });
            t.line(s3, format!("x = i·m + j = {}·{} + {} = {}", i, m, j, x));
            t.result("x", x);
            return Ok(t);
        }
        gamma = rem_euclid(gamma * factor, p);
    }
    t.table(s3, Table { headers, rows });
    Err(CalcError::KeineLoesung("Kein x gefunden".into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bsgs() {
        let t = solve(5, 8, 23).unwrap();
        let x: i128 = t.result[0].1.parse().unwrap();
        assert_eq!(mod_pow(5, x, 23).unwrap(), 8);
    }
}
