use crate::ParsedResponse;

pub async fn print(response: &ParsedResponse) {
    let ParsedResponse { name, company, date } = response;
    println!("Name: {}, Company: {}, date: {}", name, company, date);
}
