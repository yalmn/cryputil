use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum CalcError {
    UngueltigeEingabe(String),
    DivisionDurchNull,
    KeinInverses(String),
    NichtPrim(String),
    KeineLoesung(String),
    Bereich(String),
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcError::UngueltigeEingabe(s) => write!(f, "Ungültige Eingabe: {}", s),
            CalcError::DivisionDurchNull => write!(f, "Division durch Null"),
            CalcError::KeinInverses(s) => write!(f, "Kein multiplikatives Inverses: {}", s),
            CalcError::NichtPrim(s) => write!(f, "Wert ist nicht prim: {}", s),
            CalcError::KeineLoesung(s) => write!(f, "Keine Lösung gefunden: {}", s),
            CalcError::Bereich(s) => write!(f, "Wert außerhalb des erlaubten Bereichs: {}", s),
        }
    }
}

impl std::error::Error for CalcError {}

pub type CalcResult<T> = Result<T, CalcError>;
