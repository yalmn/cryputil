use crate::core::error::{CalcError, CalcResult};
use crate::core::math::{factorize, gcd, mod_inv, mod_pow};
use crate::core::trace::{Table, Trace};

// Input: n, e (öffentlich), y (Geheimtext), phi (optional: gegebenes phi(n) zur Verifikation)
// Calc:  Faktorisiere n, leite phi(n) und d ab, entschlüssele y und vergleiche optional mit gegebenem phi(n)
// Output: Trace mit privatem Schlüssel d und Klartext x
pub fn run(n: i128, e: i128, y: i128, phi_geg: Option<i128>) -> CalcResult<Trace> {
    if n <= 1 {
        return Err(CalcError::Bereich("n muss > 1 sein".into()));
    }
    if y < 0 || y >= n {
        return Err(CalcError::Bereich("y muss in [0, n) liegen".into()));
    }
    let mut t = Trace::new("RSA: aus (n, e) und y → d und x");
    t.input("n", n);
    t.input("e", e);
    t.input("y (Geheimtext)", y);
    if let Some(phi) = phi_geg {
        t.input("phi(n) (gegeben)", phi);
    }

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
        format!("phi(n) = (p - 1)(q - 1) = {} · {} = {}", p - 1, q - 1, phi),
    );

    let s3 = t.step("a) Privater Schlüssel d = e^(-1) mod phi(n)");
    if gcd(e, phi) != 1 {
        return Err(CalcError::UngueltigeEingabe(format!(
            "gcd(e, phi(n)) = {} ≠ 1",
            gcd(e, phi)
        )));
    }
    let d = mod_inv(e, phi)?;
    t.line(s3, format!("gcd(e, phi(n)) = 1 ✓"));
    t.line(
        s3,
        format!("d = e^(-1) mod phi(n) = {}^(-1) mod {} = {}", e, phi, d),
    );
    t.line(s3, format!("Probe: e · d mod phi(n) = {}", (e * d) % phi));

    let s4 = t.step("a) Entschlüsselung x = y^d mod n");
    let x = mod_pow(y, d, n)?;
    t.line(s4, format!("x = y^d mod n = {}^{} mod {} = {}", y, d, n, x));

    let s5 = t.step("b) Vorgehen, wenn phi(n) gegeben ist");
    t.line(
        s5,
        "Ohne Faktorisierung von n wird d direkt als modulares Inverses berechnet:",
    );
    t.line(s5, "  d = e^(-1) mod phi(n)");
    t.line(s5, "  x = y^d mod n");
    t.line(
        s5,
        "Die Faktoren p, q werden nicht benötigt, weil phi(n) genügt, um d zu bestimmen.",
    );

    if let Some(phi_v) = phi_geg {
        let s6 = t.step("b) Verifikation mit gegebenem phi(n)");
        t.line(s6, format!("Gegeben: phi(n) = {}", phi_v));
        t.line(
            s6,
            format!("Berechnet aus Faktorisierung: phi(n) = {}", phi),
        );
        if phi_v != phi {
            t.line(
                s6,
                "Werte stimmen NICHT überein – Eingabe oder Faktorisierung prüfen.",
            );
            return Err(CalcError::KeineLoesung("phi-Werte inkonsistent".into()));
        }
        t.line(s6, "phi-Werte stimmen überein ✓");
        if gcd(e, phi_v) != 1 {
            return Err(CalcError::UngueltigeEingabe(format!(
                "gcd(e, phi(n)) = {} ≠ 1",
                gcd(e, phi_v)
            )));
        }
        let d_alt = mod_inv(e, phi_v)?;
        let x_alt = mod_pow(y, d_alt, n)?;
        t.line(s6, format!("d (aus gegebenem phi) = {}", d_alt));
        t.line(s6, format!("x (aus gegebenem phi) = y^d mod n = {}", x_alt));
        if d_alt == d && x_alt == x {
            t.line(s6, "Lösung aus a) bestätigt ✓");
        } else {
            t.line(s6, "Abweichung festgestellt!");
        }
    }

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
    fn test_aufgabe() {
        let t = run(1010573, 83, 539875, Some(1008540)).unwrap();
        let d = t
            .result
            .iter()
            .find(|(k, _)| k.starts_with("d "))
            .unwrap()
            .1
            .clone();
        let x = t
            .result
            .iter()
            .find(|(k, _)| k.starts_with("x "))
            .unwrap()
            .1
            .clone();
        let n = 1010573i128;
        let e = 83i128;
        let y = 539875i128;
        let d_n: i128 = d.parse().unwrap();
        let x_n: i128 = x.parse().unwrap();
        assert_eq!((e * d_n) % 1008540, 1);
        assert_eq!(mod_pow(y, d_n, n).unwrap(), x_n);
    }
}
