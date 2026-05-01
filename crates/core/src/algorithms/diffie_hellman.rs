use crate::core::error::{CalcError, CalcResult};
use crate::core::math::{is_prime, mod_pow};
use crate::core::trace::Trace;

// Input: p (prim), g, a (privat A), b (privat B)
// Calc:  Diffie-Hellman-Schlüsselaustausch
// Output: Trace mit gemeinsamem Schlüssel
pub fn key_exchange(p: i128, g: i128, a: i128, b: i128) -> CalcResult<Trace> {
    if !is_prime(p) {
        return Err(CalcError::NichtPrim(p.to_string()));
    }
    let mut t = Trace::new("Diffie-Hellman-Schlüsselaustausch");
    t.input("p", p);
    t.input("g", g);
    t.input("a", a);
    t.input("b", b);

    let s1 = t.step("Öffentliche Werte");
    let big_a = mod_pow(g, a, p)?;
    let big_b = mod_pow(g, b, p)?;
    t.line(s1, format!("A = g^a mod p = {}", big_a));
    t.line(s1, format!("B = g^b mod p = {}", big_b));

    let s2 = t.step("Gemeinsamer Schlüssel");
    let k_a = mod_pow(big_b, a, p)?;
    let k_b = mod_pow(big_a, b, p)?;
    t.line(s2, format!("K_A = B^a mod p = {}", k_a));
    t.line(s2, format!("K_B = A^b mod p = {}", k_b));
    if k_a != k_b {
        return Err(CalcError::KeineLoesung(
            "Schlüssel stimmen nicht überein".into(),
        ));
    }
    t.result("Gemeinsamer Schlüssel K", k_a);
    Ok(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dh() {
        let t = key_exchange(23, 5, 6, 15).unwrap();
        assert_eq!(t.result[0].1, "2");
    }
}
