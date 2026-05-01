use crate::core::error::{CalcError, CalcResult};
use crate::core::math::{gcd, is_prime, mod_pow, order_mod};
use crate::core::trace::{Table, Trace};

// Input: g, p (p prim, 1 < g < p)
// Calc:  Ordnung von g und Auflistung der erzeugten Untergruppe
// Output: Trace
pub fn subgroup(g: i128, p: i128) -> CalcResult<Trace> {
    if !is_prime(p) {
        return Err(CalcError::NichtPrim(p.to_string()));
    }
    if gcd(g, p) != 1 {
        return Err(CalcError::Bereich("g muss teilerfremd zu p sein".into()));
    }
    let mut t = Trace::new("Zyklische Untergruppe in (Z/pZ)*");
    t.input("g", g);
    t.input("p", p);

    let s1 = t.step("Ordnung von g bestimmen");
    let ord = order_mod(g, p)?;
    t.line(s1, format!("ord(g) = {}", ord));

    let s2 = t.step("Erzeugte Untergruppe");
    let headers = vec!["k".into(), "g^k mod p".into()];
    let mut rows = Vec::new();
    for k in 0..ord {
        let v = mod_pow(g, k, p)?;
        rows.push(vec![k.to_string(), v.to_string()]);
    }
    t.table(s2, Table { headers, rows });

    t.result("Ordnung", ord);
    Ok(t)
}

// Input: p (prim)
// Calc:  alle primitive Wurzeln modulo p
// Output: Trace
pub fn primitive_roots(p: i128) -> CalcResult<Trace> {
    if !is_prime(p) {
        return Err(CalcError::NichtPrim(p.to_string()));
    }
    let mut t = Trace::new("Primitive Wurzeln modulo p");
    t.input("p", p);
    let phi = p - 1;
    let s1 = t.step("Suche nach primitiven Wurzeln");
    t.line(s1, format!("phi(p) = p - 1 = {}", phi));
    let mut roots = Vec::new();
    for g in 2..p {
        if order_mod(g, p)? == phi {
            roots.push(g);
        }
    }
    let headers = vec!["g".into(), "ord(g)".into()];
    let rows: Vec<Vec<String>> = roots
        .iter()
        .map(|g| vec![g.to_string(), phi.to_string()])
        .collect();
    t.table(s1, Table { headers, rows });
    t.result(
        "Primitive Wurzeln",
        roots
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(", "),
    );
    Ok(t)
}
