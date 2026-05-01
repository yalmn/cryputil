use cryputil_core::core::trace::{Step, Table, Trace};

// Input: Tabelle
// Calc:  Spaltenbreiten berechnen
// Output: Vec<usize>
fn column_widths(table: &Table) -> Vec<usize> {
    let mut widths: Vec<usize> = table.headers.iter().map(|h| h.chars().count()).collect();
    for row in &table.rows {
        for (i, cell) in row.iter().enumerate() {
            if i < widths.len() {
                let len = cell.chars().count();
                if len > widths[i] {
                    widths[i] = len;
                }
            }
        }
    }
    widths
}

// Input: Tabelle, Einrückung
// Calc:  Tabelle als ASCII rendern
// Output: -
fn print_table(table: &Table, indent: &str) {
    let widths = column_widths(table);
    let header: Vec<String> = table
        .headers
        .iter()
        .enumerate()
        .map(|(i, h)| format!("{:width$}", h, width = widths[i]))
        .collect();
    println!("{}{}", indent, header.join(" | "));
    let sep: Vec<String> = widths.iter().map(|w| "-".repeat(*w)).collect();
    println!("{}{}", indent, sep.join("-+-"));
    for row in &table.rows {
        let cells: Vec<String> = row
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let w = if i < widths.len() { widths[i] } else { 0 };
                format!("{:width$}", c, width = w)
            })
            .collect();
        println!("{}{}", indent, cells.join(" | "));
    }
}

// Input: Schritt
// Calc:  einzelnen Schritt formatieren und drucken
// Output: -
fn print_step(step: &Step) {
    println!();
    println!("[{}] {}", step.number, step.title);
    for line in &step.lines {
        println!("    {}", line);
    }
    if let Some(table) = &step.table {
        println!();
        print_table(table, "    ");
    }
}

// Input: Trace
// Calc:  vollständige Ausgabe inkl. aller Schritte
// Output: -
pub fn render(trace: &Trace) {
    println!();
    println!("Algorithmus: {}", trace.algorithm);
    if !trace.inputs.is_empty() {
        println!();
        println!("Eingaben:");
        for (k, v) in &trace.inputs {
            println!("  {} = {}", k, v);
        }
    }
    if !trace.steps.is_empty() {
        println!();
        println!("Schritte:");
        for step in &trace.steps {
            print_step(step);
        }
    }
    if !trace.result.is_empty() {
        println!();
        println!("Ergebnis:");
        for (k, v) in &trace.result {
            println!("  {} = {}", k, v);
        }
    }
    println!();
}

// Input: Trace
// Calc:  Kurzfassung nur mit Ergebnis
// Output: -
pub fn render_summary(trace: &Trace) {
    println!();
    println!("Algorithmus: {}", trace.algorithm);
    if !trace.result.is_empty() {
        println!();
        println!("Ergebnis:");
        for (k, v) in &trace.result {
            println!("  {} = {}", k, v);
        }
    }
    println!();
}

// Input: Fehlermeldung
// Calc:  rote/standard Fehlerausgabe
// Output: -
pub fn render_error(err: &dyn std::fmt::Display) {
    println!();
    println!("Fehler: {}", err);
    println!();
}
