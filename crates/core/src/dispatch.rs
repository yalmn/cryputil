use serde_json::Value;

use crate::algorithms::ecc::Point;
use crate::algorithms::{
    diffie_hellman, ecc, elgamal, elgamal_signature, fiat_shamir, rsa, rsa_signature,
    shamir_three_pass,
};
use crate::analysis::{
    bsgs, columnar_transposition, fermat_factorization, pollard_rho_dlog, pollard_rho_factor,
};
use crate::core::error::{CalcError, CalcResult};
use crate::core::trace::Trace;
use crate::modulo::{cyclic_groups, inverse_additive, inverse_multiplicative, operations};
use crate::playbooks::{
    dh_geg_g_p_alpha_beta_ges_k, dh_ges_schluessel, elgamal_geg_kpub_ges_kpriv,
    elgamal_geg_kpub_ges_priv_verf, elgamal_geg_sig_kpub_ges_legitsig, elgamal_mult_homomorph,
    rsa_geg_kpub_ges_kpriv_plain, rsa_geg_kpub_kpriv_encm_ges_kpriv_to_kpub,
    rsa_geg_pub_enc_ges_priv_plain,
};

// Input: Eingabewert (Value), Schlüssel
// Calc:  i128 aus Number- oder String-Repräsentation parsen
// Output: i128 oder Fehler
fn pi(v: &Value, key: &str) -> CalcResult<i128> {
    let x = v
        .get(key)
        .ok_or_else(|| CalcError::UngueltigeEingabe(format!("Parameter '{}' fehlt", key)))?;
    if let Some(s) = x.as_str() {
        s.parse::<i128>()
            .map_err(|_| CalcError::UngueltigeEingabe(format!("'{}' ist keine Ganzzahl", key)))
    } else if let Some(n) = x.as_i64() {
        Ok(n as i128)
    } else if let Some(n) = x.as_u64() {
        Ok(n as i128)
    } else {
        Err(CalcError::UngueltigeEingabe(format!(
            "Parameter '{}' muss Zahl oder Zahlen-String sein",
            key
        )))
    }
}

// Input: Wert, Schlüssel
// Calc:  optionalen i128 lesen (fehlend oder null → None)
// Output: Option<i128>
fn opt_i(v: &Value, key: &str) -> CalcResult<Option<i128>> {
    match v.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(_) => Ok(Some(pi(v, key)?)),
    }
}

// Input: Wert, Schlüssel
// Calc:  String-Parameter lesen
// Output: String
fn ps(v: &Value, key: &str) -> CalcResult<String> {
    v.get(key)
        .and_then(|x| x.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| CalcError::UngueltigeEingabe(format!("Parameter '{}' fehlt", key)))
}

// Input: Kommando-Name, Parameter (JSON)
// Calc:  Whitelist erlaubter Kommandos auswerten
// Output: Trace oder Fehler
pub fn run(command: &str, params: &Value) -> CalcResult<Trace> {
    match command {
        // Modulo
        "mod.add" => operations::add(pi(params, "a")?, pi(params, "b")?, pi(params, "n")?),
        "mod.sub" => operations::sub(pi(params, "a")?, pi(params, "b")?, pi(params, "n")?),
        "mod.mul" => operations::mul(pi(params, "a")?, pi(params, "b")?, pi(params, "n")?),
        "mod.pow" => operations::pow(pi(params, "a")?, pi(params, "e")?, pi(params, "n")?),
        "mod.inv_add" => inverse_additive::additive_inverse(pi(params, "a")?, pi(params, "n")?),
        "mod.inv_mul" => {
            inverse_multiplicative::multiplicative_inverse(pi(params, "a")?, pi(params, "n")?)
        }
        "mod.subgroup" => cyclic_groups::subgroup(pi(params, "g")?, pi(params, "p")?),
        "mod.primitive_roots" => cyclic_groups::primitive_roots(pi(params, "p")?),

        // RSA
        "rsa.keygen" => rsa::keygen(pi(params, "p")?, pi(params, "q")?, pi(params, "e")?),
        "rsa.encrypt" => rsa::encrypt(pi(params, "n")?, pi(params, "e")?, pi(params, "m")?),
        "rsa.decrypt" => rsa::decrypt(pi(params, "n")?, pi(params, "d")?, pi(params, "c")?),
        "rsa.sign" => rsa_signature::sign(pi(params, "n")?, pi(params, "d")?, pi(params, "m")?),
        "rsa.verify" => rsa_signature::verify(
            pi(params, "n")?,
            pi(params, "e")?,
            pi(params, "m")?,
            pi(params, "s")?,
        ),

        // ElGamal
        "elgamal.encrypt" => elgamal::encrypt(
            pi(params, "p")?,
            pi(params, "g")?,
            pi(params, "x")?,
            pi(params, "m")?,
            pi(params, "k")?,
        ),
        "elgamal.decrypt" => elgamal::decrypt(
            pi(params, "p")?,
            pi(params, "x")?,
            pi(params, "c1")?,
            pi(params, "c2")?,
        ),
        "elgamal.sign" => elgamal_signature::sign(
            pi(params, "p")?,
            pi(params, "g")?,
            pi(params, "d")?,
            pi(params, "m")?,
            pi(params, "k")?,
        ),
        "elgamal.verify" => elgamal_signature::verify(
            pi(params, "p")?,
            pi(params, "g")?,
            pi(params, "e")?,
            pi(params, "m")?,
            pi(params, "r")?,
            pi(params, "s")?,
        ),

        // Diffie-Hellman
        "dh.exchange" => diffie_hellman::key_exchange(
            pi(params, "p")?,
            pi(params, "g")?,
            pi(params, "a")?,
            pi(params, "b")?,
        ),

        // Shamir
        "shamir.three_pass" => shamir_three_pass::run(
            pi(params, "p")?,
            pi(params, "m")?,
            pi(params, "a")?,
            pi(params, "b")?,
        ),

        // ECC
        "ecc.add" => ecc::add_trace(
            pi(params, "a")?,
            pi(params, "b")?,
            pi(params, "p")?,
            Point::Affine(pi(params, "px")?, pi(params, "py")?),
            Point::Affine(pi(params, "qx")?, pi(params, "qy")?),
        ),
        "ecc.scalar" => ecc::scalar_trace(
            pi(params, "a")?,
            pi(params, "b")?,
            pi(params, "p")?,
            pi(params, "k")?,
            Point::Affine(pi(params, "px")?, pi(params, "py")?),
        ),

        // Fiat-Shamir
        "fiat_shamir.round" => fiat_shamir::round(
            pi(params, "n")?,
            pi(params, "s")?,
            pi(params, "k")?,
            pi(params, "e")?,
        ),

        // Analyse
        "bsgs" => bsgs::solve(pi(params, "g")?, pi(params, "h")?, pi(params, "p")?),
        "pollard_rho.factor" => pollard_rho_factor::factor(pi(params, "n")?),
        "pollard_rho.dlog" => {
            pollard_rho_dlog::solve(pi(params, "g")?, pi(params, "h")?, pi(params, "p")?)
        }
        "fermat.factor" => fermat_factorization::factor(pi(params, "n")?),
        "transposition.decrypt" => columnar_transposition::decrypt(&ps(params, "text")?),

        // Playbooks
        "pb.elgamal_mult_homomorph" => elgamal_mult_homomorph::run(
            pi(params, "p")?,
            pi(params, "g")?,
            pi(params, "e")?,
            pi(params, "d")?,
            pi(params, "a1")?,
            pi(params, "b1")?,
            pi(params, "a")?,
            pi(params, "b")?,
        ),
        "pb.rsa_priv_from_pub_y" => rsa_geg_pub_enc_ges_priv_plain::run(
            pi(params, "n")?,
            pi(params, "e")?,
            pi(params, "y")?,
            opt_i(params, "phi")?,
        ),
        "pb.dh_k_from_g_p_alpha_beta" => dh_geg_g_p_alpha_beta_ges_k::run(
            pi(params, "g")?,
            pi(params, "p")?,
            pi(params, "alpha")?,
            pi(params, "beta")?,
        ),
        "pb.elgamal_d_from_kpub" => elgamal_geg_kpub_ges_priv_verf::run(
            pi(params, "p")?,
            pi(params, "g")?,
            pi(params, "e")?,
        ),
        "pb.rsa_check_d" => rsa_geg_kpub_kpriv_encm_ges_kpriv_to_kpub::run(
            pi(params, "n")?,
            pi(params, "e")?,
            pi(params, "d")?,
            opt_i(params, "y")?,
        ),
        "pb.elgamal_sig_verify" => elgamal_geg_sig_kpub_ges_legitsig::run(
            pi(params, "p")?,
            pi(params, "g")?,
            pi(params, "e")?,
            pi(params, "m")?,
            pi(params, "r")?,
            pi(params, "s")?,
        ),
        "pb.rsa_pcap_decrypt" => {
            rsa_geg_kpub_ges_kpriv_plain::run(pi(params, "n")?, pi(params, "e")?, pi(params, "y")?)
        }
        "pb.elgamal_pcap_decrypt" => elgamal_geg_kpub_ges_kpriv::run(
            pi(params, "p")?,
            pi(params, "g")?,
            pi(params, "e")?,
            pi(params, "a")?,
            pi(params, "b")?,
        ),
        "pb.dh_pcap_shared" => dh_ges_schluessel::run(
            pi(params, "p")?,
            pi(params, "g")?,
            pi(params, "alpha")?,
            pi(params, "beta")?,
            opt_i(params, "c")?,
        ),

        other => Err(CalcError::UngueltigeEingabe(format!(
            "Unbekanntes Kommando '{}'",
            other
        ))),
    }
}

#[derive(Debug, Clone)]
pub struct CommandSpec {
    pub name: &'static str,
    pub params: &'static [&'static str],
    pub description: &'static str,
}

// Input: -
// Calc:  Liste der erlaubten Kommandos für help-Anzeige
// Output: Slice der Specs
pub fn commands() -> &'static [CommandSpec] {
    &[
        CommandSpec {
            name: "mod.add",
            params: &["a", "b", "n"],
            description: "Modulare Addition",
        },
        CommandSpec {
            name: "mod.sub",
            params: &["a", "b", "n"],
            description: "Modulare Subtraktion",
        },
        CommandSpec {
            name: "mod.mul",
            params: &["a", "b", "n"],
            description: "Modulare Multiplikation",
        },
        CommandSpec {
            name: "mod.pow",
            params: &["a", "e", "n"],
            description: "Schnelle Modulare Exponentiation",
        },
        CommandSpec {
            name: "mod.inv_add",
            params: &["a", "n"],
            description: "Additives Inverses",
        },
        CommandSpec {
            name: "mod.inv_mul",
            params: &["a", "n"],
            description: "Multiplikatives Inverses",
        },
        CommandSpec {
            name: "mod.subgroup",
            params: &["g", "p"],
            description: "Zyklische Untergruppe",
        },
        CommandSpec {
            name: "mod.primitive_roots",
            params: &["p"],
            description: "Primitive Wurzeln",
        },
        CommandSpec {
            name: "rsa.keygen",
            params: &["p", "q", "e"],
            description: "RSA-Schlüsselerzeugung",
        },
        CommandSpec {
            name: "rsa.encrypt",
            params: &["n", "e", "m"],
            description: "RSA-Verschlüsselung",
        },
        CommandSpec {
            name: "rsa.decrypt",
            params: &["n", "d", "c"],
            description: "RSA-Entschlüsselung",
        },
        CommandSpec {
            name: "rsa.sign",
            params: &["n", "d", "m"],
            description: "RSA-Signatur erzeugen",
        },
        CommandSpec {
            name: "rsa.verify",
            params: &["n", "e", "m", "s"],
            description: "RSA-Signatur prüfen",
        },
        CommandSpec {
            name: "elgamal.encrypt",
            params: &["p", "g", "x", "m", "k"],
            description: "ElGamal-Verschlüsselung",
        },
        CommandSpec {
            name: "elgamal.decrypt",
            params: &["p", "x", "c1", "c2"],
            description: "ElGamal-Entschlüsselung",
        },
        CommandSpec {
            name: "elgamal.sign",
            params: &["p", "g", "d", "m", "k"],
            description: "ElGamal-Signatur erzeugen",
        },
        CommandSpec {
            name: "elgamal.verify",
            params: &["p", "g", "e", "m", "r", "s"],
            description: "ElGamal-Signatur prüfen",
        },
        CommandSpec {
            name: "dh.exchange",
            params: &["p", "g", "a", "b"],
            description: "Diffie-Hellman",
        },
        CommandSpec {
            name: "shamir.three_pass",
            params: &["p", "m", "a", "b"],
            description: "Shamir Three-Pass",
        },
        CommandSpec {
            name: "ecc.add",
            params: &["a", "b", "p", "px", "py", "qx", "qy"],
            description: "ECC: Punktaddition",
        },
        CommandSpec {
            name: "ecc.scalar",
            params: &["a", "b", "p", "k", "px", "py"],
            description: "ECC: Skalarmultiplikation",
        },
        CommandSpec {
            name: "fiat_shamir.round",
            params: &["n", "s", "k", "e"],
            description: "Fiat-Shamir-Runde",
        },
        CommandSpec {
            name: "bsgs",
            params: &["g", "h", "p"],
            description: "Baby-Step-Giant-Step",
        },
        CommandSpec {
            name: "pollard_rho.factor",
            params: &["n"],
            description: "Pollard-Rho-Faktorisierung",
        },
        CommandSpec {
            name: "pollard_rho.dlog",
            params: &["g", "h", "p"],
            description: "Pollard-Rho-Logarithmus",
        },
        CommandSpec {
            name: "fermat.factor",
            params: &["n"],
            description: "Fermat-Faktorisierung",
        },
        CommandSpec {
            name: "transposition.decrypt",
            params: &["text"],
            description: "Spaltentransposition",
        },
        CommandSpec {
            name: "pb.elgamal_mult_homomorph",
            params: &["p", "g", "e", "d", "a1", "b1", "a", "b"],
            description: "Playbook: ElGamal Homomorphismus",
        },
        CommandSpec {
            name: "pb.rsa_priv_from_pub_y",
            params: &["n", "e", "y", "phi?"],
            description: "Playbook: RSA aus (n,e,y) → d",
        },
        CommandSpec {
            name: "pb.dh_k_from_g_p_alpha_beta",
            params: &["g", "p", "alpha", "beta"],
            description: "Playbook: DH K aus α,β",
        },
        CommandSpec {
            name: "pb.elgamal_d_from_kpub",
            params: &["p", "g", "e"],
            description: "Playbook: ElGamal d aus Kpub",
        },
        CommandSpec {
            name: "pb.rsa_check_d",
            params: &["n", "e", "d", "y?"],
            description: "Playbook: passt d zu Kpub?",
        },
        CommandSpec {
            name: "pb.elgamal_sig_verify",
            params: &["p", "g", "e", "m", "r", "s"],
            description: "Playbook: ElGamal-Signatur",
        },
        CommandSpec {
            name: "pb.rsa_pcap_decrypt",
            params: &["n", "e", "y"],
            description: "Playbook: RSA aus pcap",
        },
        CommandSpec {
            name: "pb.elgamal_pcap_decrypt",
            params: &["p", "g", "e", "a", "b"],
            description: "Playbook: ElGamal aus pcap",
        },
        CommandSpec {
            name: "pb.dh_pcap_shared",
            params: &["p", "g", "alpha", "beta", "c?"],
            description: "Playbook: DH aus pcap",
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_mod_add_via_dispatch() {
        let trace = run("mod.add", &json!({"a": 7, "b": 5, "n": 12})).unwrap();
        assert_eq!(trace.result[0].1, "0");
    }

    #[test]
    fn test_string_numbers_for_large_values() {
        let trace = run("rsa.encrypt", &json!({"n": "143", "e": "7", "m": "9"})).unwrap();
        assert!(!trace.result.is_empty());
    }

    #[test]
    fn test_unknown_command() {
        let err = run("nope", &json!({})).unwrap_err();
        assert!(err.to_string().contains("Unbekanntes Kommando"));
    }

    #[test]
    fn test_missing_param() {
        let err = run("mod.add", &json!({"a": 1, "b": 2})).unwrap_err();
        assert!(err.to_string().contains("'n'"));
    }

    #[test]
    fn test_commands_listed() {
        let cmds = commands();
        assert!(cmds.iter().any(|c| c.name == "rsa.encrypt"));
        assert!(cmds.len() > 20);
    }
}
