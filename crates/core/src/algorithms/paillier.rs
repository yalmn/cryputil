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

// Input: u (∈ Z_{n²} mit u ≡ 1 mod n), n
// Calc:  L-Funktion L(u) = (u − 1) / n
// Output: ganzzahliger Quotient oder Fehler
fn ell(u: i128, n: i128) -> CalcResult<i128> {
    if (u - 1).rem_euclid(n) != 0 {
        return Err(CalcError::Bereich(
            "L(u): Voraussetzung u ≡ 1 mod n verletzt".into(),
        ));
    }
    Ok((u - 1) / n)
}

// Input: p, q (verschiedene Primzahlen)
// Calc:  Paillier-Schlüsselerzeugung mit Standard-Generator g = n + 1
// Output: Trace mit (n, g) öffentlich und (λ, μ) privat
pub fn keygen(p: i128, q: i128) -> CalcResult<Trace> {
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
    let mut t = Trace::new("Paillier-Schlüsselerzeugung");
    t.input("p", p);
    t.input("q", q);

    let s1 = t.step("Modulus n und n²");
    let n = p * q;
    let n2 = n * n;
    t.line(s1, format!("n = p · q = {}", n));
    t.line(s1, format!("n² = {}", n2));

    let s2 = t.step("Carmichael-/Gruppenordnung λ = lcm(p−1, q−1)");
    let lambda = lcm(p - 1, q - 1);
    t.line(s2, format!("λ = lcm({}, {}) = {}", p - 1, q - 1, lambda));

    let s3 = t.step("Generator g = n + 1 (Standardwahl)");
    let g = n + 1;
    t.line(s3, format!("g = n + 1 = {}", g));

    let s4 = t.step("μ = (L(g^λ mod n²))⁻¹ mod n");
    let g_lambda = mod_pow(g, lambda, n2)?;
    t.line(s4, format!("g^λ mod n² = {}", g_lambda));
    let l_val = ell(g_lambda, n)?;
    t.line(s4, format!("L(g^λ mod n²) = (u − 1) / n = {}", l_val));
    let mu = mod_inv(l_val, n)?;
    t.line(s4, format!("μ = L(...)⁻¹ mod n = {}", mu));

    t.result("Öffentlicher Schlüssel (n, g)", format!("({}, {})", n, g));
    t.result("Privater Schlüssel (λ, μ)", format!("({}, {})", lambda, mu));
    Ok(t)
}

// Input: n, g, m (∈ [0, n)), r (gcd(r, n) = 1)
// Calc:  c = g^m · r^n mod n²
// Output: Trace mit Zwischenwerten und Chiffrat
pub fn encrypt(n: i128, g: i128, m: i128, r: i128) -> CalcResult<Trace> {
    if n <= 1 {
        return Err(CalcError::Bereich("n muss > 1 sein".into()));
    }
    if m < 0 || m >= n {
        return Err(CalcError::Bereich("m muss in [0, n) liegen".into()));
    }
    if gcd(r, n) != 1 {
        return Err(CalcError::UngueltigeEingabe(format!(
            "gcd(r, n) = {} ≠ 1",
            gcd(r, n)
        )));
    }
    let mut t = Trace::new("Paillier-Verschlüsselung");
    t.input("n", n);
    t.input("g", g);
    t.input("m", m);
    t.input("r", r);

    let n2 = n * n;
    let s1 = t.step("Bestandteile berechnen");
    let g_m = mod_pow(g, m, n2)?;
    let r_n = mod_pow(r, n, n2)?;
    t.line(s1, format!("g^m mod n² = {}^{} mod {} = {}", g, m, n2, g_m));
    t.line(s1, format!("r^n mod n² = {}^{} mod {} = {}", r, n, n2, r_n));

    let s2 = t.step("Chiffrat zusammenfügen");
    let c = rem_euclid(g_m * r_n, n2);
    t.line(s2, format!("c = g^m · r^n mod n² = {}", c));
    t.result("Chiffrat c", c);
    Ok(t)
}

// Input: n, λ, μ, c
// Calc:  m = L(c^λ mod n²) · μ mod n
// Output: Trace mit Klartext
pub fn decrypt(n: i128, lambda: i128, mu: i128, c: i128) -> CalcResult<Trace> {
    if n <= 1 {
        return Err(CalcError::Bereich("n muss > 1 sein".into()));
    }
    let n2 = n * n;
    let mut t = Trace::new("Paillier-Entschlüsselung");
    t.input("n", n);
    t.input("λ", lambda);
    t.input("μ", mu);
    t.input("c", c);

    let s1 = t.step("c^λ mod n²");
    let c_lambda = mod_pow(c, lambda, n2)?;
    t.line(
        s1,
        format!("c^λ mod n² = {}^{} mod {} = {}", c, lambda, n2, c_lambda),
    );

    let s2 = t.step("L-Funktion auswerten");
    let l_val = ell(c_lambda, n)?;
    t.line(
        s2,
        format!("L(c^λ mod n²) = ({} − 1) / {} = {}", c_lambda, n, l_val),
    );

    let s3 = t.step("Multiplikation mit μ");
    let m = rem_euclid(l_val * mu, n);
    t.line(
        s3,
        format!(
            "m = L(...) · μ mod n = {} · {} mod {} = {}",
            l_val, mu, n, m
        ),
    );
    t.result("Klartext m", m);
    Ok(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keygen_small() {
        let t = keygen(5, 7).unwrap();
        // erwartete Werte: n=35, λ=12, μ=3
        assert!(t
            .result
            .iter()
            .any(|(k, v)| k.contains("Öffentlicher") && v.contains("35") && v.contains("36")));
        assert!(t
            .result
            .iter()
            .any(|(k, v)| k.contains("Privater") && v.contains("12") && v.contains("3")));
    }

    #[test]
    fn test_roundtrip() {
        // n=35, g=36, λ=12, μ=3
        let m = 10i128;
        let r = 4i128;
        let enc = encrypt(35, 36, m, r).unwrap();
        let c: i128 = enc.result[0].1.parse().unwrap();
        let dec = decrypt(35, 12, 3, c).unwrap();
        assert_eq!(dec.result[0].1, m.to_string());
    }

    #[test]
    fn test_rejects_bad_random() {
        // gcd(7, 35) = 7 ≠ 1
        assert!(encrypt(35, 36, 4, 7).is_err());
    }

    #[test]
    fn test_rejects_m_out_of_range() {
        assert!(encrypt(35, 36, 35, 4).is_err());
    }

    #[test]
    fn test_rejects_equal_primes() {
        assert!(keygen(7, 7).is_err());
    }
}
