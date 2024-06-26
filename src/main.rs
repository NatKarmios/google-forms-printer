use anyhow::Result;
use std::{cmp::Ordering, fs, io::stdin};
use colorize::AnsiColor;

mod cfg;
mod client;
mod parse;
mod print;
mod state;
mod util;
mod pdf;

use cfg::*;
use client::*;
use parse::*;
use print::*;
use state::*;
use pdf::*;

async fn do_poll(state: &mut State) -> Result<()> {
    let responses = match state.get_responses().await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{}", e);
            return Ok(());
        }
    };

    let len = responses.len();
    if len > 0 {
        println!("Got {} new response{}.", len, (if len == 1 { "" } else { "s" }));
    }

    let mut last_handled = state.last_handled.to_owned();
    for response in responses {
        let filename = gen_pdf(&response)?;
        print_pdf(&filename, &state)?;
        if response.date.cmp(&last_handled) == Ordering::Greater {
            last_handled = response.date.clone();
        }
    }
    state.last_handled = last_handled;
    Ok(())
}

async fn run() -> Result<()> {
    fs::create_dir_all("./cfg").unwrap();
    let mut state = State::new().await?;
    println!("{}", "Good to go!".green());
    loop {
        do_poll(&mut state).await?;
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    }
}

#[tokio::main]
async fn main() {
    let _ = enable_ansi_support::enable_ansi_support();
    match run().await {
        Ok(_) => (),
        Err(e) => eprintln!("{:?}", e),
    }
    println!("Press enter to quit.");
    stdin().read_line(&mut String::new()).unwrap();
}
