use crate::core::error::{CalcError, CalcResult};
use crate::core::math::{gcd, is_prime, mod_inv, mod_pow, rem_euclid};
use crate::core::trace::{Table, Trace};

// Input: g, h, p (prim)
// Calc:  Pollard-Rho zur Berechnung von x mit g^x ≡ h mod p
// Output: Trace
pub fn solve(g: i128, h: i128, p: i128) -> CalcResult<Trace> {
    if !is_prime(p) {
        return Err(CalcError::NichtPrim(p.to_string()));
    }
    let n = p - 1;
    let mut t = Trace::new("Pollard-Rho für diskreten Logarithmus");
    t.input("g", g);
    t.input("h", h);
    t.input("p", p);

    let s = t.step("Iteration mit Partition (S1, S2, S3)");
    let f = |x: i128, a: i128, b: i128| -> (i128, i128, i128) {
        match rem_euclid(x, 3) {
            0 => (
                rem_euclid(x * x, p),
                rem_euclid(2 * a, n),
                rem_euclid(2 * b, n),
            ),
            1 => (rem_euclid(x * g, p), rem_euclid(a + 1, n), b),
            _ => (rem_euclid(x * h, p), a, rem_euclid(b + 1, n)),
        }
    };
    let headers = vec![
        "i".into(),
        "x".into(),
        "a".into(),
        "b".into(),
        "X".into(),
        "A".into(),
        "B".into(),
    ];
    let mut rows: Vec<Vec<String>> = Vec::new();
    let (mut x, mut a, mut b) = (1i128, 0i128, 0i128);
    let (mut x2, mut a2, mut b2) = (1i128, 0i128, 0i128);
    for i in 1..=100_000i128 {
        let (nx, na, nb) = f(x, a, b);
        x = nx;
        a = na;
        b = nb;
        let (nx, na, nb) = f(x2, a2, b2);
        let (nx, na, nb) = f(nx, na, nb);
        x2 = nx;
        a2 = na;
        b2 = nb;
        if rows.len() < 50 {
            rows.push(vec![
                i.to_string(),
                x.to_string(),
                a.to_string(),
                b.to_string(),
                x2.to_string(),
                a2.to_string(),
                b2.to_string(),
            ]);
        }
        if x == x2 {
            t.table(s, Table { headers, rows });
            let r = rem_euclid(b - b2, n);
            if r == 0 {
                return Err(CalcError::KeineLoesung("triviale Kollision".into()));
            }
            let g_div = gcd(r, n);
            if g_div != 1 {
                let s2 = t.step("Mehrere Lösungen prüfen");
                let n_p = n / g_div;
                let r_p = r / g_div;
                let lhs = rem_euclid(a2 - a, n) / g_div;
                let inv = mod_inv(rem_euclid(r_p, n_p), n_p)?;
                let x0 = rem_euclid(lhs * inv, n_p);
                for k in 0..g_div {
                    let cand = rem_euclid(x0 + k * n_p, n);
                    if mod_pow(g, cand, p)? == rem_euclid(h, p) {
                        t.line(s2, format!("Kandidat x = {} bestätigt.", cand));
                        t.result("x", cand);
                        return Ok(t);
                    }
                }
                return Err(CalcError::KeineLoesung("kein Kandidat passt".into()));
            }
            let inv = mod_inv(r, n)?;
            let x_sol = rem_euclid((a2 - a) * inv, n);
            t.line(
                s,
                format!("x = (a' - a) · (b - b')^(-1) mod (p-1) = {}", x_sol),
            );
            t.result("x", x_sol);
            return Ok(t);
        }
    }
    t.table(s, Table { headers, rows });
    Err(CalcError::KeineLoesung("Iterationsgrenze erreicht".into()))
}
