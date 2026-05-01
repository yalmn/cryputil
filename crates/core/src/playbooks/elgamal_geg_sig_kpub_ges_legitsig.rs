use crate::core::error::{CalcError, CalcResult};
use crate::core::math::{is_prime, mod_pow, rem_euclid};
use crate::core::trace::Trace;

// Input: p (prim), g, e (öffentlicher Schlüssel), m (Nachricht), r, s (Signatur)
// Calc:  Verifikation einer ElGamal-Signatur per Vergleich e^r · r^s mod p mit g^m mod p
// Output: Trace mit Bewertung „gültig" oder „ungültig"
pub fn run(p: i128, g: i128, e: i128, m: i128, r: i128, s: i128) -> CalcResult<Trace> {
    if !is_prime(p) {
        return Err(CalcError::NichtPrim(p.to_string()));
    }
    let mut t = Trace::new("ElGamal-Signatur: Verifikation");
    t.input("p", p);
    t.input("g", g);
    t.input("e", e);
    t.input("m", m);
    t.input("r", r);
    t.input("s", s);

    let s1 = t.step("Notwendige Schritte zur Verifikation");
    t.line(
        s1,
        "Eine ElGamal-Signatur (r, s) für Nachricht m gilt als gültig, wenn:",
    );
    t.line(s1, "  1. 0 < r < p");
    t.line(s1, "  2. 0 < s < p − 1");
    t.line(
        s1,
        "  3. v1 := e^r · r^s mod p   gleich   v2 := g^m mod p ist.",
    );
    t.line(
        s1,
        "Trifft (3) zu, gehört (r, s) zum öffentlichen Schlüssel (p, g, e) und zu m.",
    );

    let s2 = t.step("1. Bereichsprüfung von r und s");
    let r_ok = r > 0 && r < p;
    let s_ok = s > 0 && s < p - 1;
    t.line(
        s2,
        format!(
            "0 < r < p: 0 < {} < {} → {}",
            r,
            p,
            if r_ok { "ok" } else { "verletzt" }
        ),
    );
    t.line(
        s2,
        format!(
            "0 < s < p − 1: 0 < {} < {} → {}",
            s,
            p - 1,
            if s_ok { "ok" } else { "verletzt" }
        ),
    );
    if !r_ok || !s_ok {
        t.result("Signatur gültig", false);
        return Ok(t);
    }

    let s3 = t.step("2. v1 = e^r · r^s mod p");
    let er = mod_pow(e, r, p)?;
    let rs = mod_pow(r, s, p)?;
    let v1 = rem_euclid(er * rs, p);
    t.line(s3, format!("e^r mod p = {}^{} mod {} = {}", e, r, p, er));
    t.line(s3, format!("r^s mod p = {}^{} mod {} = {}", r, s, p, rs));
    t.line(
        s3,
        format!("v1 = e^r · r^s mod p = {} · {} mod {} = {}", er, rs, p, v1),
    );

    let s4 = t.step("3. v2 = g^m mod p");
    let v2 = mod_pow(g, m, p)?;
    t.line(
        s4,
        format!("v2 = g^m mod p = {}^{} mod {} = {}", g, m, p, v2),
    );

    let s5 = t.step("4. Vergleich v1 = v2");
    let valid = v1 == v2;
    t.line(s5, format!("v1 = {},  v2 = {}", v1, v2));
    if valid {
        t.line(s5, "v1 = v2 ✓ – Signatur ist gültig.");
    } else {
        t.line(s5, "v1 ≠ v2 – Signatur ist ungültig.");
    }

    t.result("v1 = e^r · r^s mod p", v1);
    t.result("v2 = g^m mod p", v2);
    t.result("Signatur gültig", valid);
    Ok(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aufgabe() {
        let t = run(31, 7, 14, 15, 28, 29).unwrap();
        let valid = t
            .result
            .iter()
            .find(|(k, _)| k.starts_with("Signatur gültig"))
            .unwrap()
            .1
            .clone();
        assert_eq!(valid, "true");
    }

    #[test]
    fn test_ungueltige_signatur() {
        let t = run(31, 7, 14, 15, 28, 30).unwrap();
        let valid = t
            .result
            .iter()
            .find(|(k, _)| k.starts_with("Signatur gültig"))
            .unwrap()
            .1
            .clone();
        assert_eq!(valid, "false");
    }
}
