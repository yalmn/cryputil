use crate::core::error::{CalcError, CalcResult};
use crate::core::math::{factorize, mod_pow, rem_euclid};
use crate::core::trace::{Table, Trace};

// Input: n, e (öffentlich), d (vermutet privat), y (optional: Beispiel-Geheimtext)
// Calc:  Prüfung, ob d zu (n, e) passt: phi(n)-Test und Round-Trip-Test über y
// Output: Trace mit Bewertung „passt" oder „passt nicht"
pub fn run(n: i128, e: i128, d: i128, y: Option<i128>) -> CalcResult<Trace> {
    if n <= 1 {
        return Err(CalcError::Bereich("n muss > 1 sein".into()));
    }
    let mut t = Trace::new("RSA: prüfen, ob privater Schlüssel d zu Kpub = (n, e) passt");
    t.input("n", n);
    t.input("e", e);
    t.input("d", d);
    if let Some(y_v) = y {
        t.input("y (Geheimtext)", y_v);
    }

    let s1 = t.step("Vorgehen");
    t.line(
        s1,
        "Ein privater RSA-Schlüssel d ist genau dann gültig, wenn",
    );
    t.line(s1, "  e · d ≡ 1 mod phi(n)");
    t.line(s1, "gilt. Das wird hier auf zwei Wegen geprüft:");
    t.line(
        s1,
        "  (1) Faktorisierung von n → phi(n) → Test e·d mod phi(n) = 1",
    );
    t.line(
        s1,
        "  (2) Round-Trip mit gegebenem y: x = y^d mod n und anschließend x^e mod n = y",
    );

    let s2 = t.step("(1) Faktorisierung von n");
    let factors = factorize(n);
    let headers = vec!["Primfaktor".into(), "Exponent".into()];
    let rows: Vec<Vec<String>> = factors
        .iter()
        .map(|(p, ex)| vec![p.to_string(), ex.to_string()])
        .collect();
    t.table(s2, Table { headers, rows });
    if factors.len() != 2 || factors[0].1 != 1 || factors[1].1 != 1 {
        return Err(CalcError::UngueltigeEingabe(
            "n ist kein Produkt zweier verschiedener Primzahlen".into(),
        ));
    }
    let p = factors[0].0;
    let q = factors[1].0;
    let phi = (p - 1) * (q - 1);
    t.line(
        s2,
        format!("p = {},  q = {},  phi(n) = (p−1)(q−1) = {}", p, q, phi),
    );

    let s3 = t.step("(1) Test e · d mod phi(n) = 1");
    let probe = rem_euclid(e * d, phi);
    t.line(s3, format!("e · d = {} · {} = {}", e, d, e * d));
    t.line(s3, format!("e · d mod phi(n) = {}", probe));
    let phi_ok = probe == 1;
    if phi_ok {
        t.line(s3, "e · d ≡ 1 mod phi(n) ✓");
    } else {
        t.line(s3, "e · d ≢ 1 mod phi(n) – d passt nicht zu (n, e).");
    }

    let mut roundtrip_ok: Option<bool> = None;
    if let Some(y_v) = y {
        let s4 = t.step("(2) Round-Trip mit gegebenem y");
        if y_v < 0 || y_v >= n {
            return Err(CalcError::Bereich("y muss in [0, n) liegen".into()));
        }
        let x = mod_pow(y_v, d, n)?;
        let y_back = mod_pow(x, e, n)?;
        t.line(
            s4,
            format!("x = y^d mod n = {}^{} mod {} = {}", y_v, d, n, x),
        );
        t.line(
            s4,
            format!("x^e mod n = {}^{} mod {} = {}", x, e, n, y_back),
        );
        let ok = y_back == y_v;
        roundtrip_ok = Some(ok);
        if ok {
            t.line(
                s4,
                "Re-Verschlüsselung ergibt wieder y ✓ – d passt zu (n, e).",
            );
            t.result("Wiederhergestellter Klartext x", x);
        } else {
            t.line(
                s4,
                "Re-Verschlüsselung ergibt nicht y – d passt nicht zu (n, e).",
            );
        }
    }

    let s5 = t.step("Gesamtergebnis");
    let passt = phi_ok && roundtrip_ok.unwrap_or(true);
    t.line(
        s5,
        format!(
            "phi(n)-Test: {}",
            if phi_ok {
                "bestanden"
            } else {
                "nicht bestanden"
            }
        ),
    );
    if let Some(rt) = roundtrip_ok {
        t.line(
            s5,
            format!(
                "Round-Trip-Test: {}",
                if rt { "bestanden" } else { "nicht bestanden" }
            ),
        );
    }
    t.line(
        s5,
        if passt {
            "Der private Schlüssel d passt zum öffentlichen Schlüssel (n, e)."
        } else {
            "Der private Schlüssel d passt NICHT zum öffentlichen Schlüssel (n, e)."
        },
    );

    t.result("phi(n)", phi);
    t.result("e · d mod phi(n)", probe);
    t.result("d passt zu (n, e)", passt);
    Ok(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aufgabe() {
        let t = run(444149, 37, 59833, Some(56688)).unwrap();
        let passt = t
            .result
            .iter()
            .find(|(k, _)| k.starts_with("d passt"))
            .unwrap()
            .1
            .clone();
        assert_eq!(passt, "true");
    }

    #[test]
    fn test_falscher_schluessel() {
        let t = run(444149, 37, 12345, Some(56688)).unwrap();
        let passt = t
            .result
            .iter()
            .find(|(k, _)| k.starts_with("d passt"))
            .unwrap()
            .1
            .clone();
        assert_eq!(passt, "false");
    }
}
