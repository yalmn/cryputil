use crate::core::error::{CalcError, CalcResult};
use crate::core::math::{is_prime, mod_inv, mod_pow, rem_euclid};
use crate::core::trace::Trace;

// Input: p (prim), g, x (privat), m (Klartext < p), k (ephemeral)
// Calc:  ElGamal-Verschlüsselung
// Output: Trace
pub fn encrypt(p: i128, g: i128, x: i128, m: i128, k: i128) -> CalcResult<Trace> {
    if !is_prime(p) {
        return Err(CalcError::NichtPrim(p.to_string()));
    }
    if m < 0 || m >= p {
        return Err(CalcError::Bereich("m muss in [0, p) liegen".into()));
    }
    let mut t = Trace::new("ElGamal-Verschlüsselung");
    t.input("p", p);
    t.input("g", g);
    t.input("x (privat)", x);
    t.input("m", m);
    t.input("k (ephemeral)", k);

    let s1 = t.step("Öffentlicher Schlüssel");
    let y = mod_pow(g, x, p)?;
    t.line(s1, format!("y = g^x mod p = {}", y));

    let s2 = t.step("Chiffrat");
    let c1 = mod_pow(g, k, p)?;
    let c2 = rem_euclid(m * mod_pow(y, k, p)?, p);
    t.line(s2, format!("c1 = g^k mod p = {}", c1));
    t.line(s2, format!("c2 = m · y^k mod p = {}", c2));

    t.result("Chiffrat (c1, c2)", format!("({}, {})", c1, c2));
    Ok(t)
}

// Input: p, x (privat), c1, c2
// Calc:  ElGamal-Entschlüsselung
// Output: Trace
pub fn decrypt(p: i128, x: i128, c1: i128, c2: i128) -> CalcResult<Trace> {
    let mut t = Trace::new("ElGamal-Entschlüsselung");
    t.input("p", p);
    t.input("x", x);
    t.input("c1", c1);
    t.input("c2", c2);

    let s1 = t.step("Inverses Maskierungselement");
    let s = mod_pow(c1, x, p)?;
    let s_inv = mod_inv(s, p)?;
    t.line(s1, format!("s = c1^x mod p = {}", s));
    t.line(s1, format!("s^(-1) mod p = {}", s_inv));

    let s2 = t.step("Klartext");
    let m = rem_euclid(c2 * s_inv, p);
    t.line(s2, format!("m = c2 · s^(-1) mod p = {}", m));
    t.result("Klartext m", m);
    Ok(t)
}
