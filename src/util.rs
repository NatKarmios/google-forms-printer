use std::{fmt::Display, io::{stdin, stdout, Write}};

pub const CONFIG_FILE: &str = "./cfg/config.json";
pub const SECRETS_FILE: &str = "./cfg/secrets.json";
pub const TOKENS_FILE: &str = "./cfg/tokens.json";

pub const SCOPES: &'static [&str] = &[
    "https://www.googleapis.com/auth/forms.responses.readonly",
    "https://www.googleapis.com/auth/forms.body.readonly",
];

fn read_usize() -> Option<usize> {
    let mut s = String::new();
    stdout().flush().unwrap();
    stdin().read_line(&mut s).ok()?;
    s.trim().parse::<usize>().ok()
}

pub fn choose_named<T>(names: Vec<String>, mut items: Vec<T>) -> T {
    for (i, name) in names.iter().enumerate() {
        println!("[{}] {}", i+1, name);
    }
    let mut i = 0;
    while i < 1 || i > items.len() {
        print!("Enter a number ({}-{}): ", 1, items.len());
        if let Some(n) = read_usize() {
            i = n;
        }
    }
    items.remove(i-1)
}

pub fn choose<T : Display + Copy>(items: &Vec<T>) -> T {
    for (i, item) in items.iter().enumerate() {
        println!("[{}] {}", i+1, item);
    }
    let mut i = 0;
    while i < 1 || i > items.len() {
        print!("Enter a number ({}-{}): ", 1, items.len());
        if let Some(n) = read_usize() {
            i = n;
        }
    }
    items[i-1]
}
