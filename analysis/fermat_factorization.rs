use crate::core::error::{CalcError, CalcResult};
use crate::core::math::isqrt;
use crate::core::trace::{Table, Trace};

// Input: n (ungerade, > 1)
// Calc:  Fermat-Faktorisierung sucht a, b mit a² - b² = n
// Output: Trace mit Faktoren
pub fn factor(n: i128) -> CalcResult<Trace> {
    if n <= 1 {
        return Err(CalcError::Bereich("n muss > 1 sein".into()));
    }
    if n % 2 == 0 {
        return Err(CalcError::UngueltigeEingabe("n muss ungerade sein".into()));
    }
    let mut t = Trace::new("Fermat-Faktorisierung");
    t.input("n", n);

    let s = t.step("Suche nach a mit a² - n = b²");
    let mut a = isqrt(n);
    if a * a < n {
        a += 1;
    }
    let headers = vec!["a".into(), "a² - n".into(), "b² ?".into()];
    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut iters = 0;
    loop {
        let diff = a * a - n;
        let b = isqrt(diff);
        let is_sq = b * b == diff;
        rows.push(vec![
            a.to_string(),
            diff.to_string(),
            if is_sq {
                format!("{} = {}²", diff, b)
            } else {
                "nein".into()
            },
        ]);
        if is_sq {
            t.table(s, Table { headers, rows });
            let p = a - b;
            let q = a + b;
            t.line(s, format!("n = (a - b)(a + b) = {} · {}", p, q));
            t.result("Faktor 1", p);
            t.result("Faktor 2", q);
            return Ok(t);
        }
        a += 1;
        iters += 1;
        if iters > 1_000_000 {
            t.table(s, Table { headers, rows });
            return Err(CalcError::KeineLoesung("Iterationsgrenze erreicht".into()));
        }
    }
}
