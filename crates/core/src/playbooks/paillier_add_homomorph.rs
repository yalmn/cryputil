use crate::core::error::{CalcError, CalcResult};
use crate::core::math::{gcd, is_prime, mod_inv, mod_pow, rem_euclid};
use crate::core::trace::Trace;

// Input: a, b
// Calc:  kleinstes gemeinsames Vielfaches
// Output: lcm(a, b)
fn lcm(a: i128, b: i128) -> i128 {
    if a == 0 || b == 0 {
        0
    } else {
        (a / gcd(a, b)) * b
    }
}

// Input: p, q (verschiedene Primzahlen), m1, m2 ∈ [0, n), r1, r2 mit gcd(r_i, n) = 1
// Calc:  Vollständiger Durchlauf des additiven Homomorphismus von Paillier:
//        Schlüsselableitung mit g = n+1, beide Chiffrate, Kombination c = c1·c2 mod n²
//        und Entschlüsselung zu (m1 + m2) mod n
// Output: Trace mit allen Zwischenwerten und Probe
pub fn run(p: i128, q: i128, m1: i128, m2: i128, r1: i128, r2: i128) -> CalcResult<Trace> {
    if !is_prime(p) {
        return Err(CalcError::NichtPrim(format!("p = {}", p)));
    }
    if !is_prime(q) {
        return Err(CalcError::NichtPrim(format!("q = {}", q)));
    }
    if p == q {
        return Err(CalcError::UngueltigeEingabe(
            "p und q müssen verschieden sein".into(),
        ));
    }
    let n = p * q;
    if m1 < 0 || m1 >= n {
        return Err(CalcError::Bereich("m1 muss in [0, n) liegen".into()));
    }
    if m2 < 0 || m2 >= n {
        return Err(CalcError::Bereich("m2 muss in [0, n) liegen".into()));
    }
    if gcd(r1, n) != 1 {
        return Err(CalcError::UngueltigeEingabe(format!(
            "gcd(r1, n) = {} ≠ 1",
            gcd(r1, n)
        )));
    }
    if gcd(r2, n) != 1 {
        return Err(CalcError::UngueltigeEingabe(format!(
            "gcd(r2, n) = {} ≠ 1",
            gcd(r2, n)
        )));
    }

    let mut t = Trace::new("Paillier: Additiver Homomorphismus");
    t.input("p", p);
    t.input("q", q);
    t.input("m1", m1);
    t.input("m2", m2);
    t.input("r1", r1);
    t.input("r2", r2);

    let n2 = n * n;
    let g = n + 1;
    let lambda = lcm(p - 1, q - 1);

    let s1 = t.step("Schlüsselableitung");
    t.line(s1, format!("n = p · q = {}", n));
    t.line(s1, format!("n² = {}", n2));
    t.line(s1, format!("λ = lcm(p−1, q−1) = {}", lambda));
    t.line(s1, format!("g = n + 1 = {}", g));
    let g_lambda = mod_pow(g, lambda, n2)?;
    let l_keygen = (g_lambda - 1) / n;
    let mu = mod_inv(l_keygen, n)?;
    t.line(s1, format!("g^λ mod n² = {}", g_lambda));
    t.line(s1, format!("L(g^λ mod n²) = {}", l_keygen));
    t.line(s1, format!("μ = L(...)⁻¹ mod n = {}", mu));

    let s2 = t.step("Eigenschaft des additiven Homomorphismus");
    t.line(s2, "E(m1) · E(m2) mod n² = E((m1 + m2) mod n).");
    t.line(
        s2,
        "Anschaulich: das Produkt zweier Paillier-Chiffren entschlüsselt zur Summe der Klartexte.",
    );

    let s3 = t.step("Chiffrat c1 = E(m1, r1)");
    let g_m1 = mod_pow(g, m1, n2)?;
    let r1_n = mod_pow(r1, n, n2)?;
    let c1 = rem_euclid(g_m1 * r1_n, n2);
    t.line(s3, format!("g^m1 mod n² = {}", g_m1));
    t.line(s3, format!("r1^n mod n² = {}", r1_n));
    t.line(s3, format!("c1 = g^m1 · r1^n mod n² = {}", c1));

    let s4 = t.step("Chiffrat c2 = E(m2, r2)");
    let g_m2 = mod_pow(g, m2, n2)?;
    let r2_n = mod_pow(r2, n, n2)?;
    let c2 = rem_euclid(g_m2 * r2_n, n2);
    t.line(s4, format!("g^m2 mod n² = {}", g_m2));
    t.line(s4, format!("r2^n mod n² = {}", r2_n));
    t.line(s4, format!("c2 = g^m2 · r2^n mod n² = {}", c2));

    let s5 = t.step("Kombinieren: c = c1 · c2 mod n²");
    let c = rem_euclid(c1 * c2, n2);
    t.line(
        s5,
        format!("c = c1 · c2 mod n² = {} · {} mod {} = {}", c1, c2, n2, c),
    );

    let s6 = t.step("Entschlüsselung von c");
    let c_lambda = mod_pow(c, lambda, n2)?;
    let l_c = (c_lambda - 1) / n;
    let m_total = rem_euclid(l_c * mu, n);
    t.line(s6, format!("c^λ mod n² = {}", c_lambda));
    t.line(s6, format!("L(c^λ mod n²) = {}", l_c));
    t.line(
        s6,
        format!(
            "m_total = L(...) · μ mod n = {} · {} mod {} = {}",
            l_c, mu, n, m_total
        ),
    );

    let s7 = t.step("Probe");
    let expected = rem_euclid(m1 + m2, n);
    t.line(
        s7,
        format!(
            "(m1 + m2) mod n = ({} + {}) mod {} = {}",
            m1, m2, n, expected
        ),
    );
    t.line(
        s7,
        format!(
            "Entschlüsselung ergab m_total = {} (erwartet: {})",
            m_total, expected
        ),
    );

    t.result("c1", c1);
    t.result("c2", c2);
    t.result("c = c1 · c2 mod n²", c);
    t.result("Entschlüsselung m_total", m_total);
    t.result("(m1 + m2) mod n", expected);
    Ok(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_homomorphism_holds() {
        // p=5, q=7, n=35; m1=3, m2=7, r1=4, r2=9 (beide gcd-frei zu 35)
        let t = run(5, 7, 3, 7, 4, 9).unwrap();
        let m_total = t
            .result
            .iter()
            .find(|(k, _)| k.starts_with("Entschlüsselung"))
            .unwrap()
            .1
            .clone();
        let expected = t
            .result
            .iter()
            .find(|(k, _)| k.starts_with("(m1 + m2)"))
            .unwrap()
            .1
            .clone();
        assert_eq!(m_total, expected);
        assert_eq!(m_total, "10");
    }

    #[test]
    fn test_wraps_around_n() {
        // m1+m2 ≥ n: muss mod n umlaufen
        let t = run(5, 7, 30, 10, 4, 9).unwrap();
        let m_total = t
            .result
            .iter()
            .find(|(k, _)| k.starts_with("Entschlüsselung"))
            .unwrap()
            .1
            .clone();
        // (30+10) mod 35 = 5
        assert_eq!(m_total, "5");
    }

    #[test]
    fn test_rejects_bad_random() {
        // gcd(7, 35) = 7
        assert!(run(5, 7, 3, 4, 7, 9).is_err());
    }
}
