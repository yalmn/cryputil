use std::io::{self, Write};

use crate::core::validation::parse_int;

// Input: Aufforderungstext
// Calc:  Eingabezeile lesen
// Output: getrimmter String
pub fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    let _ = io::stdout().flush();
    let mut buf = String::new();
    if io::stdin().read_line(&mut buf).is_err() {
        return String::new();
    }
    buf.trim().to_string()
}

// Input: Aufforderung
// Calc:  fragt wiederholt nach einer ganzen Zahl
// Output: i128
pub fn ask_int(label: &str) -> i128 {
    loop {
        let line = read_line(&format!("  {}: ", label));
        match parse_int(&line) {
            Ok(v) => return v,
            Err(e) => println!("    {}", e),
        }
    }
}

// Input: Aufforderung, Untergrenze (inkl.)
// Calc:  fragt wiederholt nach einer Ganzzahl >= lo
// Output: i128
pub fn ask_int_min(label: &str, lo: i128) -> i128 {
    loop {
        let v = ask_int(label);
        if v >= lo {
            return v;
        }
        println!("    Wert muss >= {} sein.", lo);
    }
}

// Input: Aufforderung
// Calc:  fragt nach Ja/Nein
// Output: bool
pub fn ask_yes_no(label: &str) -> bool {
    loop {
        let line = read_line(&format!("  {} [j/n]: ", label)).to_lowercase();
        match line.as_str() {
            "j" | "ja" | "y" | "yes" => return true,
            "n" | "nein" | "no" => return false,
            _ => println!("    Bitte j oder n eingeben."),
        }
    }
}
