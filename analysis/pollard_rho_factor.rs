use crate::core::error::{CalcError, CalcResult};
use crate::core::math::{gcd, rem_euclid};
use crate::core::trace::{Table, Trace};

// Input: n (zusammengesetzt, > 1)
// Calc:  Pollard-Rho-Faktorisierung mit Floyd-Zykluserkennung
// Output: Trace mit nicht-trivialem Faktor
pub fn factor(n: i128) -> CalcResult<Trace> {
    if n <= 1 {
        return Err(CalcError::Bereich("n muss > 1 sein".into()));
    }
    let mut t = Trace::new("Pollard-Rho-Faktorisierung");
    t.input("n", n);
    if n % 2 == 0 {
        let s = t.step("Geradzahligkeit");
        t.line(s, "n ist gerade, Faktor 2 gefunden.");
        t.result("Faktor", 2i128);
        return Ok(t);
    }
    let s = t.step("Iteration f(x) = x² + 1 mod n");
    let headers = vec![
        "i".into(),
        "x".into(),
        "y".into(),
        "|x - y|".into(),
        "gcd".into(),
    ];
    let mut rows: Vec<Vec<String>> = Vec::new();
    let f = |x: i128| -> i128 { rem_euclid(x * x + 1, n) };
    let mut x: i128 = 2;
    let mut y: i128 = 2;
    let mut d: i128 = 1;
    let mut i = 0;
    while d == 1 {
        x = f(x);
        y = f(f(y));
        d = gcd((x - y).abs(), n);
        i += 1;
        rows.push(vec![
            i.to_string(),
            x.to_string(),
            y.to_string(),
            (x - y).abs().to_string(),
            d.to_string(),
        ]);
        if i > 100_000 {
            t.table(s, Table { headers, rows });
            return Err(CalcError::KeineLoesung("Iterationsgrenze erreicht".into()));
        }
    }
    t.table(s, Table { headers, rows });
    if d == n {
        return Err(CalcError::KeineLoesung(
            "nur trivialer Faktor gefunden".into(),
        ));
    }
    t.result("Faktor", d);
    t.result("Co-Faktor", n / d);
    Ok(t)
}
