use crate::cli::menu;
use crate::core::trace::Trace;
use crate::render::text;

// Input: -
// Calc:  Hauptschleife mit letztem Trace im Speicher
// Output: -
pub fn run() {
    let mut last: Option<Trace> = None;
    loop {
        let choice = menu::show_main();
        let result = match choice.as_str() {
            "1" => menu::modulo_menu(),
            "2" => menu::crypto_menu(),
            "3" => menu::ident_menu(),
            "4" => menu::analysis_menu(),
            "5" => menu::playbooks_menu(),
            "6" => {
                match &last {
                    Some(t) => text::render(t),
                    None => {
                        println!();
                        println!("Es liegen noch keine Schritte vor.");
                        println!();
                    }
                }
                None
            }
            "0" => {
                println!();
                println!("Auf Wiedersehen.");
                return;
            }
            _ => {
                println!();
                println!("Ungültige Auswahl.");
                None
            }
        };
        if let Some(t) = result {
            last = Some(t);
        }
    }
}
