use anyhow::{Context, Result};
use winprint::printer::{FilePrinter, PdfiumPrinter, PrinterDevice};
use winprint::ticket::{MediaSizeTuple, PrintCapabilities, PrintTicketBuilder};
use std::path::Path;

use crate::{util::choose_named, State};

const PRINT_WIDTH: f32 = 74.0;
const PRINT_HEIGHT: f32 = 50.8;
const EPSILON: f32 = 0.1;

pub fn choose_printer() -> String {
    println!("Select which printer to use:");
    let printers = PrinterDevice::all().expect("Failed to get printers.");
    let names = printers.iter().map(|p| p.name().to_owned()).collect();
    let printer = choose_named(names, printers);
    printer.name().to_owned()
}

fn is_same_size(size: &MediaSizeTuple, width: f32, height: f32, epsilon: f32) -> bool {
    let width_diff = (size.width_in_micron() as f32 / 1000.0 - width).abs();
    let height_diff = (size.height_in_micron() as f32 / 1000.0 - height).abs();
    width_diff < epsilon && height_diff < epsilon
}

pub fn print_pdf(filename: &str, state: &State) -> Result<()> {
    let device = PrinterDevice::all().context("Failed to get printers.")?
        .into_iter().find(|p| p.name() == state.printer).with_context(|| format!("Couln't find printer '{}'", state.printer))?;
    let media_size = PrintCapabilities::fetch(&device).expect("Failed to get capabilites")
        .page_media_size().find(|s| is_same_size(&s.size(), PRINT_WIDTH, PRINT_HEIGHT, EPSILON)).context("Couldn't find suitable print size!")?;
    let mut builder = PrintTicketBuilder::new(&device).unwrap();
    builder.merge(media_size).unwrap();
    let ticket = builder.build().unwrap();
    PdfiumPrinter::new(device).print(&Path::new(filename), ticket).context("Failed to print PDF.")?;
    Ok(())
}
