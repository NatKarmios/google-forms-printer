use printers::printer::Printer;

use crate::{util::choose_named, ParsedResponse, State};

pub fn choose_printer() -> Printer {
    println!("Select which printer to use:");
    let printers = printers::get_printers();
    let names = printers.iter().map(|p| p.name.to_owned()).collect();
    let printer = choose_named(names, printers);
    printer
}

pub async fn print(response: &ParsedResponse, state: &State) {
    let msg = format!("Name: {}\nCompany: {}\n", response.name, response.company);
    state.printer.print(msg.as_bytes(), None).unwrap();
}
