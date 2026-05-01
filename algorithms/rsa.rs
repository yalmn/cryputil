use crate::core::error::{CalcError, CalcResult};
use crate::core::math::{gcd, is_prime, mod_inv, mod_pow};
use crate::core::trace::Trace;

// Input: p, q (prim), e (öffentlicher Exponent)
// Calc:  vollständiges RSA-Schlüsselpaar mit Trace
// Output: Trace
pub fn keygen(p: i128, q: i128, e: i128) -> CalcResult<Trace> {
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
    let mut t = Trace::new("RSA-Schlüsselerzeugung");
    t.input("p", p);
    t.input("q", q);
    t.input("e", e);

    let s1 = t.step("Modulus n und Eulersche Funktion");
    let n = p * q;
    let phi = (p - 1) * (q - 1);
    t.line(s1, format!("n = p · q = {}", n));
    t.line(s1, format!("phi(n) = (p-1)(q-1) = {}", phi));

    let s2 = t.step("Prüfung des öffentlichen Exponenten e");
    if gcd(e, phi) != 1 {
        return Err(CalcError::UngueltigeEingabe(format!(
            "gcd(e, phi(n)) = {} ≠ 1",
            gcd(e, phi)
        )));
    }
    t.line(s2, format!("gcd(e, phi(n)) = 1 ✓"));

    let s3 = t.step("Privater Exponent d");
    let d = mod_inv(e, phi)?;
    t.line(s3, format!("d = e^(-1) mod phi(n) = {}", d));

    t.result("Öffentlicher Schlüssel (n, e)", format!("({}, {})", n, e));
    t.result("Privater Schlüssel (n, d)", format!("({}, {})", n, d));
    Ok(t)
}

// Input: n, e, m (Klartext < n)
// Calc:  c = m^e mod n
// Output: Trace
pub fn encrypt(n: i128, e: i128, m: i128) -> CalcResult<Trace> {
    if m < 0 || m >= n {
        return Err(CalcError::Bereich(
            "Klartext m muss in [0, n) liegen".into(),
        ));
    }
    let mut t = Trace::new("RSA-Verschlüsselung");
    t.input("n", n);
    t.input("e", e);
    t.input("m", m);
    let s = t.step("Chiffrat berechnen");
    let c = mod_pow(m, e, n)?;
    t.line(s, format!("c = m^e mod n = {}^{} mod {} = {}", m, e, n, c));
    t.result("Chiffrat c", c);
    Ok(t)
}

// Input: n, d, c
// Calc:  m = c^d mod n
// Output: Trace
pub fn decrypt(n: i128, d: i128, c: i128) -> CalcResult<Trace> {
    let mut t = Trace::new("RSA-Entschlüsselung");
    t.input("n", n);
    t.input("d", d);
    t.input("c", c);
    let s = t.step("Klartext berechnen");
    let m = mod_pow(c, d, n)?;
    t.line(s, format!("m = c^d mod n = {}^{} mod {} = {}", c, d, n, m));
    t.result("Klartext m", m);
    Ok(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rsa_roundtrip() {
        let kg = keygen(11, 13, 7).unwrap();
        assert!(kg.result.iter().any(|r| r.0.contains("Privater")));
        let enc = encrypt(143, 7, 9).unwrap();
        let c: i128 = enc.result[0].1.parse().unwrap();
        let dec = decrypt(143, 103, c).unwrap();
        assert_eq!(dec.result[0].1, "9");
    }
}
