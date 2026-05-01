use crate::core::error::{CalcError, CalcResult};

// Input: a, n (n > 0)
// Calc:  a mod n im Bereich [0, n)
// Output: a mod n
pub fn rem_euclid(a: i128, n: i128) -> i128 {
    let r = a % n;
    if r < 0 {
        r + n.abs()
    } else {
        r
    }
}

// Input: a, b
// Calc:  größter gemeinsamer Teiler
// Output: gcd(a, b)
pub fn gcd(a: i128, b: i128) -> i128 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

// Input: a, b
// Calc:  erweiterter euklidischer Algorithmus, ggT und Koeffizienten
// Output: (g, x, y) mit a*x + b*y = g
pub fn ext_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x1, y1) = ext_gcd(b, a % b);
        (g, y1, x1 - (a / b) * y1)
    }
}

// Input: a, n (n > 0)
// Calc:  multiplikatives Inverses von a modulo n
// Output: a^{-1} mod n oder Fehler
pub fn mod_inv(a: i128, n: i128) -> CalcResult<i128> {
    if n <= 0 {
        return Err(CalcError::Bereich("Modulus muss > 0 sein".into()));
    }
    let (g, x, _) = ext_gcd(rem_euclid(a, n), n);
    if g != 1 {
        return Err(CalcError::KeinInverses(format!(
            "gcd({}, {}) = {} ≠ 1",
            a, n, g
        )));
    }
    Ok(rem_euclid(x, n))
}

// Input: base, exp (>= 0), modulus (> 0)
// Calc:  modulare Exponentiation per Square-and-Multiply
// Output: base^exp mod modulus
pub fn mod_pow(base: i128, exp: i128, modulus: i128) -> CalcResult<i128> {
    if modulus <= 0 {
        return Err(CalcError::Bereich("Modulus muss > 0 sein".into()));
    }
    if exp < 0 {
        let inv = mod_inv(base, modulus)?;
        return mod_pow(inv, -exp, modulus);
    }
    if modulus == 1 {
        return Ok(0);
    }
    let mut result: i128 = 1;
    let mut b = rem_euclid(base, modulus);
    let mut e = exp;
    while e > 0 {
        if e & 1 == 1 {
            result = (result * b) % modulus;
        }
        e >>= 1;
        if e > 0 {
            b = (b * b) % modulus;
        }
    }
    Ok(result)
}

// Input: n (>= 0)
// Calc:  ganzzahlige Wurzel
// Output: floor(sqrt(n))
pub fn isqrt(n: i128) -> i128 {
    if n < 0 {
        return 0;
    }
    if n < 2 {
        return n;
    }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}

// Input: n
// Calc:  einfacher Primzahltest per Probedivision
// Output: true falls n prim
pub fn is_prime(n: i128) -> bool {
    if n < 2 {
        return false;
    }
    if n < 4 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i: i128 = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

// Input: n (>= 1)
// Calc:  vollständige Primfaktorzerlegung
// Output: Liste (prim, exponent) aufsteigend
pub fn factorize(n: i128) -> Vec<(i128, u32)> {
    let mut result = Vec::new();
    let mut x = n.abs();
    let mut p: i128 = 2;
    while p * p <= x {
        if x % p == 0 {
            let mut e = 0u32;
            while x % p == 0 {
                x /= p;
                e += 1;
            }
            result.push((p, e));
        }
        p += if p == 2 { 1 } else { 2 };
    }
    if x > 1 {
        result.push((x, 1));
    }
    result
}

// Input: n (> 0)
// Calc:  Eulersche Phi-Funktion
// Output: phi(n)
pub fn euler_phi(n: i128) -> i128 {
    if n <= 0 {
        return 0;
    }
    let mut result = n;
    for (p, _) in factorize(n) {
        result = result / p * (p - 1);
    }
    result
}

// Input: g, p (p prim)
// Calc:  Ordnung von g in (Z/pZ)*
// Output: kleinstes k > 0 mit g^k ≡ 1 mod p
pub fn order_mod(g: i128, p: i128) -> CalcResult<i128> {
    if gcd(g, p) != 1 {
        return Err(CalcError::Bereich("g muss teilerfremd zu p sein".into()));
    }
    let phi = euler_phi(p);
    let mut divisors: Vec<i128> = Vec::new();
    let mut d = 1;
    while d * d <= phi {
        if phi % d == 0 {
            divisors.push(d);
            if d != phi / d {
                divisors.push(phi / d);
            }
        }
        d += 1;
    }
    divisors.sort();
    for k in divisors {
        if mod_pow(g, k, p)? == 1 {
            return Ok(k);
        }
    }
    Err(CalcError::KeineLoesung("Ordnung nicht gefunden".into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(12, 18), 6);
        assert_eq!(gcd(17, 31), 1);
    }

    #[test]
    fn test_mod_inv() {
        assert_eq!(mod_inv(3, 11).unwrap(), 4);
        assert!(mod_inv(2, 4).is_err());
    }

    #[test]
    fn test_mod_pow() {
        assert_eq!(mod_pow(5, 3, 13).unwrap(), 8);
        assert_eq!(mod_pow(2, 10, 1000).unwrap(), 24);
    }

    #[test]
    fn test_is_prime() {
        assert!(is_prime(2));
        assert!(is_prime(23));
        assert!(!is_prime(21));
    }

    #[test]
    fn test_phi() {
        assert_eq!(euler_phi(10), 4);
        assert_eq!(euler_phi(13), 12);
    }
}
