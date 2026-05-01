#[derive(Debug, Clone)]
pub struct Table {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct Step {
    pub number: usize,
    pub title: String,
    pub lines: Vec<String>,
    pub table: Option<Table>,
}

#[derive(Debug, Clone)]
pub struct Trace {
    pub algorithm: String,
    pub inputs: Vec<(String, String)>,
    pub steps: Vec<Step>,
    pub result: Vec<(String, String)>,
}

impl Trace {
    // Input: Algorithmusname
    // Calc:  leeren Trace anlegen
    // Output: neuer Trace
    pub fn new(algorithm: &str) -> Self {
        Trace {
            algorithm: algorithm.to_string(),
            inputs: Vec::new(),
            steps: Vec::new(),
            result: Vec::new(),
        }
    }

    // Input: Name, Wert als String
    // Calc:  Eingabe an Trace anhängen
    // Output: -
    pub fn input(&mut self, name: &str, value: impl ToString) {
        self.inputs.push((name.to_string(), value.to_string()));
    }

    // Input: Titel
    // Calc:  neuen Schritt anlegen und Index zurückgeben
    // Output: Index des Schritts
    pub fn step(&mut self, title: &str) -> usize {
        let n = self.steps.len() + 1;
        self.steps.push(Step {
            number: n,
            title: title.to_string(),
            lines: Vec::new(),
            table: None,
        });
        self.steps.len() - 1
    }

    // Input: Index, Textzeile
    // Calc:  Zeile dem Schritt hinzufügen
    // Output: -
    pub fn line(&mut self, idx: usize, text: impl ToString) {
        self.steps[idx].lines.push(text.to_string());
    }

    // Input: Index, Tabelle
    // Calc:  Tabelle dem Schritt zuordnen
    // Output: -
    pub fn table(&mut self, idx: usize, table: Table) {
        self.steps[idx].table = Some(table);
    }

    // Input: Name, Wert
    // Calc:  Ergebnis-Eintrag anhängen
    // Output: -
    pub fn result(&mut self, name: &str, value: impl ToString) {
        self.result.push((name.to_string(), value.to_string()));
    }
}
