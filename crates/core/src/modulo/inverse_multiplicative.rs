use crate::core::error::{CalcError, CalcResult};
use crate::core::math::rem_euclid;
use crate::core::trace::{Table, Trace};

// Input: a, n (n > 0)
// Calc:  erweiterter euklidischer Algorithmus mit Tabelle
// Output: Trace inkl. Inverses falls existent
pub fn multiplicative_inverse(a: i128, n: i128) -> CalcResult<Trace> {
    if n <= 0 {
        return Err(CalcError::Bereich("n muss > 0 sein".into()));
    }
    let mut t = Trace::new("Multiplikatives Inverses (erweiterter Euklid)");
    t.input("a", a);
    t.input("n", n);

    let s1 = t.step("Erweiterter Euklidischer Algorithmus");
    let headers = vec![
        "i".into(),
        "r_i".into(),
        "q_i".into(),
        "s_i".into(),
        "t_i".into(),
    ];
    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut r0 = n;
    let mut r1 = rem_euclid(a, n);
    let mut s0: i128 = 1;
    let mut s1v: i128 = 0;
    let mut tt0: i128 = 0;
    let mut tt1: i128 = 1;
    let mut i = 0usize;
    rows.push(vec![
        i.to_string(),
        r0.to_string(),
        "-".into(),
        s0.to_string(),
        tt0.to_string(),
    ]);
    i += 1;
    rows.push(vec![
        i.to_string(),
        r1.to_string(),
        "-".into(),
        s1v.to_string(),
        tt1.to_string(),
    ]);
    while r1 != 0 {
        let q = r0 / r1;
        let r2 = r0 - q * r1;
        let s2 = s0 - q * s1v;
        let t2 = tt0 - q * tt1;
        i += 1;
        rows.push(vec![
            i.to_string(),
            r2.to_string(),
            q.to_string(),
            s2.to_string(),
            t2.to_string(),
        ]);
        r0 = r1;
        r1 = r2;
        s0 = s1v;
        s1v = s2;
        tt0 = tt1;
        tt1 = t2;
    }
    t.table(s1, Table { headers, rows });
    t.line(s1, format!("ggT(a, n) = {}", r0));

    if r0 != 1 {
        t.line(
            s1,
            "Da gcd(a, n) ≠ 1, existiert kein multiplikatives Inverses.",
        );
        return Err(CalcError::KeinInverses(format!(
            "gcd({}, {}) = {}",
            a, n, r0
        )));
    }
    let inv = rem_euclid(tt0, n);
    let s2 = t.step("Inverses ablesen");
    t.line(s2, format!("a^(-1) mod n = {}", inv));
    let probe = rem_euclid(rem_euclid(a, n) * inv, n);
    t.line(s2, format!("Prüfung: a · a^(-1) mod n = {}", probe));
    t.result("Multiplikatives Inverses", inv);
    Ok(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inverse() {
        let t = multiplicative_inverse(3, 11).unwrap();
        assert_eq!(t.result[0].1, "4");
    }

    #[test]
    fn test_no_inverse() {
        assert!(multiplicative_inverse(2, 4).is_err());
    }
}
