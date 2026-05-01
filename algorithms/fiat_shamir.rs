use crate::core::error::{CalcError, CalcResult};
use crate::core::math::{mod_pow, rem_euclid};
use crate::core::trace::Trace;

// Input: n, s (Geheimnis), k (Commitment-Zufall), e (0/1)
// Calc:  Eine Runde Fiat-Shamir mit Commitment x = k² mod n
// Output: Trace
pub fn round(n: i128, s: i128, k: i128, e: i128) -> CalcResult<Trace> {
    if !(e == 0 || e == 1) {
        return Err(CalcError::UngueltigeEingabe("e muss 0 oder 1 sein".into()));
    }
    let mut t = Trace::new("Fiat-Shamir Identifikationsprotokoll (eine Runde)");
    t.input("n", n);
    t.input("s (Geheimnis)", s);
    t.input("k (Commitment-Zufall)", k);
    t.input("e (Challenge)", e);

    let s1 = t.step("Public Key");
    let v = mod_pow(s, 2, n)?;
    t.line(s1, format!("v = s² mod n = {}", v));

    let s2 = t.step("Commitment");
    let x = mod_pow(k, 2, n)?;
    t.line(s2, format!("x = k² mod n = {}", x));

    let s3 = t.step("Response");
    let y = if e == 0 {
        rem_euclid(k, n)
    } else {
        rem_euclid(k * s, n)
    };
    t.line(s3, format!("y = k · s^e mod n = {}", y));

    let s4 = t.step("Verifikation");
    let lhs = mod_pow(y, 2, n)?;
    let rhs = rem_euclid(x * mod_pow(v, e, n)?, n);
    t.line(s4, format!("y² mod n = {}", lhs));
    t.line(s4, format!("x · v^e mod n = {}", rhs));
    let ok = lhs == rhs;
    t.line(s4, format!("Akzeptiert: {}", ok));

    t.result("Akzeptiert", ok);
    Ok(t)
}
