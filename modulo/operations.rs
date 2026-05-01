use crate::core::error::CalcResult;
use crate::core::math::{mod_pow, rem_euclid};
use crate::core::trace::Trace;

// Input: a, b, n
// Calc:  (a + b) mod n mit Trace
// Output: Trace
pub fn add(a: i128, b: i128, n: i128) -> CalcResult<Trace> {
    let mut t = Trace::new("Modulare Addition");
    t.input("a", a);
    t.input("b", b);
    t.input("n", n);
    let s = t.step("Reduktion und Summe");
    let ar = rem_euclid(a, n);
    let br = rem_euclid(b, n);
    t.line(s, format!("a mod n = {} mod {} = {}", a, n, ar));
    t.line(s, format!("b mod n = {} mod {} = {}", b, n, br));
    let sum = rem_euclid(ar + br, n);
    t.line(
        s,
        format!("(a + b) mod n = ({} + {}) mod {} = {}", ar, br, n, sum),
    );
    t.result("Summe", sum);
    Ok(t)
}

// Input: a, b, n
// Calc:  (a - b) mod n mit Trace
// Output: Trace
pub fn sub(a: i128, b: i128, n: i128) -> CalcResult<Trace> {
    let mut t = Trace::new("Modulare Subtraktion");
    t.input("a", a);
    t.input("b", b);
    t.input("n", n);
    let s = t.step("Reduktion und Differenz");
    let ar = rem_euclid(a, n);
    let br = rem_euclid(b, n);
    t.line(s, format!("a mod n = {}", ar));
    t.line(s, format!("b mod n = {}", br));
    let diff = rem_euclid(ar - br, n);
    t.line(s, format!("(a - b) mod n = {}", diff));
    t.result("Differenz", diff);
    Ok(t)
}

// Input: a, b, n
// Calc:  (a * b) mod n mit Trace
// Output: Trace
pub fn mul(a: i128, b: i128, n: i128) -> CalcResult<Trace> {
    let mut t = Trace::new("Modulare Multiplikation");
    t.input("a", a);
    t.input("b", b);
    t.input("n", n);
    let s = t.step("Reduktion und Produkt");
    let ar = rem_euclid(a, n);
    let br = rem_euclid(b, n);
    t.line(s, format!("a mod n = {}", ar));
    t.line(s, format!("b mod n = {}", br));
    let prod = rem_euclid(ar * br, n);
    t.line(s, format!("(a * b) mod n = {}", prod));
    t.result("Produkt", prod);
    Ok(t)
}

// Input: a, e (>= 0), n
// Calc:  a^e mod n via Square-and-Multiply mit Tracetabelle
// Output: Trace
pub fn pow(a: i128, e: i128, n: i128) -> CalcResult<Trace> {
    let mut t = Trace::new("Modulare Exponentiation (Square-and-Multiply)");
    t.input("a", a);
    t.input("e", e);
    t.input("n", n);
    let s1 = t.step("Binärdarstellung des Exponenten");
    let bin = if e >= 0 {
        format!("{:b}", e)
    } else {
        format!("-{:b}", -e)
    };
    t.line(s1, format!("e = {} = {}_2", e, bin));
    let s2 = t.step("Square-and-Multiply");
    let mut headers = vec![
        "i".into(),
        "bit".into(),
        "quadrieren".into(),
        "multiplizieren".into(),
    ];
    let _ = &mut headers;
    let mut rows: Vec<Vec<String>> = Vec::new();
    if e >= 0 && e <= 1_000_000 {
        let bits: Vec<u8> = format!("{:b}", e).bytes().map(|b| b - b'0').collect();
        let mut acc: i128 = 1;
        for (i, bit) in bits.iter().enumerate() {
            let before = acc;
            acc = (acc * acc) % n;
            let after_sq = acc;
            let mult = if *bit == 1 {
                acc = (acc * rem_euclid(a, n)) % n;
                acc.to_string()
            } else {
                "-".into()
            };
            rows.push(vec![
                i.to_string(),
                bit.to_string(),
                format!("{}^2 mod n = {}", before, after_sq),
                mult,
            ]);
        }
    }
    t.table(s2, crate::core::trace::Table { headers, rows });
    let result = mod_pow(a, e, n)?;
    t.line(s2, format!("a^e mod n = {}", result));
    t.result("Potenz", result);
    Ok(t)
}
