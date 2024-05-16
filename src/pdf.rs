use std::{fs::create_dir_all, process::Command, time::{SystemTime, UNIX_EPOCH}};

use anyhow::Result;

use crate::ParsedResponse;

fn run_typst(filename: &str, name: &str, company: &str) -> Result<()> {
  let args = [
    "compile",
    "./pdf/pdf.typ",
    &format!("./tmp/{}", filename),
    "--input", &format!("name={}", name),
    "--input", &format!("company={}", company),
    "--font-path", "./pdf",
  ];
  Command::new("./pdf/typst.exe")
    .args(&args)
    .output()
    .map_err(|e| anyhow::anyhow!("Error running typst:\n./typst.exe {}\n{}", args.join(" "), e))?;
  Ok(())
}

pub fn gen_pdf(response: &ParsedResponse) -> Result<String> {
  create_dir_all("./tmp").unwrap();
  let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
  let filename = format!("{}.pdf", now);
  run_typst(&filename, &response.name, &response.company)?;
  Ok(filename)
}
