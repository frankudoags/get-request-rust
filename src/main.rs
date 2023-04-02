use error_chain::error_chain;
use std::io::Read;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Reqwest(reqwest::Error);
    }
}

fn main() -> Result<()> {
    let mut resp = reqwest::blocking::get("https://jsonplaceholder.typicode.com/users")?;
    let mut body = String::new();
    resp.read_to_string(&mut body)?;
    println!("Status: {}", resp.status());
    println!("Headers:\n{:#?}", resp.headers());

    println!("Body:\n{}", body);
    
    Ok(())
}