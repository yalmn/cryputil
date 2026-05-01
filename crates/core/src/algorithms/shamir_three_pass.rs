use crate::core::error::{CalcError, CalcResult};
use crate::core::math::{gcd, is_prime, mod_inv, mod_pow};
use crate::core::trace::Trace;

// Input: p (prim), m (Klartext), a (Alice), b (Bob)
// Calc:  Shamir-Three-Pass-Protokoll
// Output: Trace
pub fn run(p: i128, m: i128, a: i128, b: i128) -> CalcResult<Trace> {
    if !is_prime(p) {
        return Err(CalcError::NichtPrim(p.to_string()));
    }
    let phi = p - 1;
    if gcd(a, phi) != 1 {
        return Err(CalcError::UngueltigeEingabe(
            "gcd(a, p-1) muss 1 sein".into(),
        ));
    }
    if gcd(b, phi) != 1 {
        return Err(CalcError::UngueltigeEingabe(
            "gcd(b, p-1) muss 1 sein".into(),
        ));
    }
    let mut t = Trace::new("Shamir Three-Pass-Protokoll");
    t.input("p", p);
    t.input("m", m);
    t.input("a (Alice)", a);
    t.input("b (Bob)", b);

    let a_inv = mod_inv(a, phi)?;
    let b_inv = mod_inv(b, phi)?;

    let s1 = t.step("Schritt 1: Alice → Bob");
    let c1 = mod_pow(m, a, p)?;
    t.line(s1, format!("c1 = m^a mod p = {}", c1));

    let s2 = t.step("Schritt 2: Bob → Alice");
    let c2 = mod_pow(c1, b, p)?;
    t.line(s2, format!("c2 = c1^b mod p = {}", c2));

    let s3 = t.step("Schritt 3: Alice → Bob");
    let c3 = mod_pow(c2, a_inv, p)?;
    t.line(s3, format!("a^(-1) mod (p-1) = {}", a_inv));
    t.line(s3, format!("c3 = c2^(a^-1) mod p = {}", c3));

    let s4 = t.step("Bob entschlüsselt");
    let m_rec = mod_pow(c3, b_inv, p)?;
    t.line(s4, format!("b^(-1) mod (p-1) = {}", b_inv));
    t.line(s4, format!("m = c3^(b^-1) mod p = {}", m_rec));

    t.result("Wiederhergestellter Klartext", m_rec);
    Ok(t)
}
