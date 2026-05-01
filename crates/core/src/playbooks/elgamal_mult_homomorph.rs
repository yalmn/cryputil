use crate::core::error::{CalcError, CalcResult};
use crate::core::math::{is_prime, mod_inv, mod_pow, rem_euclid};
use crate::core::trace::Trace;

// Input: p (prim), g, e (öffentlicher Schlüssel), d (privat), (a1, b1) Geheimtext zu m1, (a, b) kombinierter Geheimtext
// Calc:  Multiplikativer Homomorphismus: m_total = m1 · m2 mod p, daraus m2 = m_total / m1 mod p
// Output: Trace mit m1, m_total und gesuchtem Erweiterungsfaktor m2
pub fn run(
    p: i128,
    g: i128,
    e: i128,
    d: i128,
    a1: i128,
    b1: i128,
    a_total: i128,
    b_total: i128,
) -> CalcResult<Trace> {
    if !is_prime(p) {
        return Err(CalcError::NichtPrim(p.to_string()));
    }
    let mut t = Trace::new("ElGamal: Multiplikativer Homomorphismus");
    t.input("p", p);
    t.input("g", g);
    t.input("e (öffentlich)", e);
    t.input("d (privat)", d);
    t.input("a1", a1);
    t.input("b1", b1);
    t.input("a (kombiniert)", a_total);
    t.input("b (kombiniert)", b_total);

    let s1 = t.step("Eigenschaft des multiplikativen Homomorphismus");
    t.line(
        s1,
        "(a1, b1) · (a2, b2) = (a1·a2 mod p, b1·b2 mod p) verschlüsselt m1·m2 mod p.",
    );
    t.line(
        s1,
        "Daher gilt für den kombinierten Geheimtext: Dec(a, b) = m1 · m2 mod p.",
    );

    let s2 = t.step("Entschlüsselung des Original-Chiffrats (a1, b1) → m1");
    let s_a1 = mod_pow(a1, d, p)?;
    let s_a1_inv = mod_inv(s_a1, p)?;
    let m1 = rem_euclid(b1 * s_a1_inv, p);
    t.line(
        s2,
        format!("a1^d mod p = {}^{} mod {} = {}", a1, d, p, s_a1),
    );
    t.line(s2, format!("(a1^d)^(-1) mod p = {}", s_a1_inv));
    t.line(s2, format!("m1 = b1 · (a1^d)^(-1) mod p = {}", m1));

    let s3 = t.step("Entschlüsselung des kombinierten Chiffrats (a, b) → m1·m2");
    let s_a = mod_pow(a_total, d, p)?;
    let s_a_inv = mod_inv(s_a, p)?;
    let m_total = rem_euclid(b_total * s_a_inv, p);
    t.line(
        s3,
        format!("a^d mod p = {}^{} mod {} = {}", a_total, d, p, s_a),
    );
    t.line(s3, format!("(a^d)^(-1) mod p = {}", s_a_inv));
    t.line(s3, format!("m_total = b · (a^d)^(-1) mod p = {}", m_total));

    let s4 = t.step("Erweiterungsfaktor m2 isolieren");
    if m1 == 0 {
        return Err(CalcError::KeineLoesung(
            "m1 = 0, m2 nicht eindeutig bestimmbar".into(),
        ));
    }
    let m1_inv = mod_inv(m1, p)?;
    let m2 = rem_euclid(m_total * m1_inv, p);
    t.line(s4, format!("m1^(-1) mod p = {}", m1_inv));
    t.line(
        s4,
        format!(
            "m2 = m_total · m1^(-1) mod p = {} · {} mod {} = {}",
            m_total, m1_inv, p, m2
        ),
    );

    let s5 = t.step("Probe");
    let probe = rem_euclid(m1 * m2, p);
    t.line(
        s5,
        format!(
            "m1 · m2 mod p = {} (erwartet: m_total = {})",
            probe, m_total
        ),
    );

    let s6 = t.step("Alternativer Ansatz (Aufgabe b)");
    t.line(
        s6,
        "Zerlege den kombinierten Geheimtext direkt in die Erweiterungs-Komponente:",
    );
    t.line(s6, "  a2 = a · a1^(-1) mod p,  b2 = b · b1^(-1) mod p");
    t.line(
        s6,
        "und entschlüssele anschließend (a2, b2) klassisch via m2 = b2 · (a2^d)^(-1) mod p.",
    );
    if let (Ok(a1_inv), Ok(b1_inv)) = (mod_inv(a1, p), mod_inv(b1, p)) {
        let a2 = rem_euclid(a_total * a1_inv, p);
        let b2 = rem_euclid(b_total * b1_inv, p);
        t.line(s6, format!("Konkret: a2 = {}, b2 = {}", a2, b2));
        if let Ok(s_a2) = mod_pow(a2, d, p) {
            if let Ok(inv) = mod_inv(s_a2, p) {
                let m2_alt = rem_euclid(b2 * inv, p);
                t.line(
                    s6,
                    format!("Entschlüsselung von (a2, b2) ergibt m2 = {}", m2_alt),
                );
            }
        }
    }

    t.result("m1", m1);
    t.result("m_total = m1 · m2", m_total);
    t.result("Erweiterungsfaktor m2", m2);
    Ok(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aufgabe() {
        let t = run(31, 11, 3, 17, 13, 10, 2, 2).unwrap();
        let m2 = t
            .result
            .iter()
            .find(|(k, _)| k.starts_with("Erweiterungsfaktor"))
            .unwrap()
            .1
            .clone();
        assert_eq!(m2, "21");
    }
}
