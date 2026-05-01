use crate::core::error::{CalcError, CalcResult};
use crate::core::math::{factorize, gcd, mod_inv, mod_pow};
use crate::core::trace::{Table, Trace};

// Input: n, e (öffentlich), y (Geheimtext, ggf. aus Wireshark-Mitschnitt extrahiert)
// Calc:  RSA brechen via Faktorisierung von n: phi(n) → d → x = y^d mod n
// Output: Trace mit privatem Schlüssel d und Klartext x
pub fn run(n: i128, e: i128, y: i128) -> CalcResult<Trace> {
    if n <= 1 {
        return Err(CalcError::Bereich("n muss > 1 sein".into()));
    }
    if y < 0 || y >= n {
        return Err(CalcError::Bereich("y muss in [0, n) liegen".into()));
    }
    let mut t = Trace::new("RSA: aus Kpub = (n, e) → privater Schlüssel und Klartext");
    t.input("n", n);
    t.input("e", e);
    t.input("y (Geheimtext)", y);

    let s0 = t.step("Hinweis zur Notation");
    t.line(
        s0,
        "In Mitschnitten (z. B. .pcapng) liegen Werte häufig als Hex- oder Big-Endian-Bytes vor.",
    );
    t.line(
        s0,
        "Vor der Eingabe in dieses Playbook sind n, e und y in Dezimalform umzurechnen.",
    );
    t.line(
        s0,
        "Übliche Notation: Kpub = (n, e), Kpriv = d, Geheimtext y, Klartext x mit y = x^e mod n.",
    );

    let s1 = t.step("a) Faktorisierung von n");
    let factors = factorize(n);
    let headers = vec!["Primfaktor".into(), "Exponent".into()];
    let rows: Vec<Vec<String>> = factors
        .iter()
        .map(|(p, ex)| vec![p.to_string(), ex.to_string()])
        .collect();
    t.table(s1, Table { headers, rows });
    if factors.len() != 2 || factors[0].1 != 1 || factors[1].1 != 1 {
        return Err(CalcError::UngueltigeEingabe(
            "n ist kein Produkt zweier verschiedener Primzahlen".into(),
        ));
    }
    let p = factors[0].0;
    let q = factors[1].0;
    t.line(s1, format!("p = {},  q = {},  n = p · q = {}", p, q, p * q));

    let s2 = t.step("a) Eulersche Phi-Funktion phi(n)");
    let phi = (p - 1) * (q - 1);
    t.line(
        s2,
        format!("phi(n) = (p − 1)(q − 1) = {} · {} = {}", p - 1, q - 1, phi),
    );

    let s3 = t.step("a) Privater Schlüssel d = e^(-1) mod phi(n)");
    if gcd(e, phi) != 1 {
        return Err(CalcError::UngueltigeEingabe(format!(
            "gcd(e, phi(n)) = {} ≠ 1",
            gcd(e, phi)
        )));
    }
    let d = mod_inv(e, phi)?;
    t.line(s3, "gcd(e, phi(n)) = 1 ✓");
    t.line(
        s3,
        format!("d = e^(-1) mod phi(n) = {}^(-1) mod {} = {}", e, phi, d),
    );
    t.line(s3, format!("Probe: e · d mod phi(n) = {}", (e * d) % phi));

    let s4 = t.step("a) Entschlüsselung x = y^d mod n");
    let x = mod_pow(y, d, n)?;
    t.line(s4, format!("x = y^d mod n = {}^{} mod {} = {}", y, d, n, x));

    let s5 = t.step("Zusammenfassung");
    t.line(s5, format!("Kpub = (n, e) = ({}, {})", n, e));
    t.line(s5, format!("Kpriv = d = {}", d));
    t.line(s5, format!("Klartext x = {}", x));

    t.result("p", p);
    t.result("q", q);
    t.result("phi(n)", phi);
    t.result("d (privat)", d);
    t.result("x (Klartext)", x);
    Ok(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip() {
        let p = 263i128;
        let q = 311i128;
        let n = p * q;
        let e = 17i128;
        let phi = (p - 1) * (q - 1);
        let m = 12345i128;
        let y = mod_pow(m, e, n).unwrap();
        let t = run(n, e, y).unwrap();
        let x = t
            .result
            .iter()
            .find(|(k, _)| k.starts_with("x "))
            .unwrap()
            .1
            .clone();
        assert_eq!(x, m.to_string());
        let d = t
            .result
            .iter()
            .find(|(k, _)| k.starts_with("d "))
            .unwrap()
            .1
            .parse::<i128>()
            .unwrap();
        assert_eq!((e * d) % phi, 1);
    }
}
