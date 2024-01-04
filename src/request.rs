#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait Request {
    fn get(&self, url: String) -> Result<String, Box<dyn std::error::Error>> {
        println!("Request to {}", url);
        let result = reqwest::blocking::get(url)?.text()?;
        Ok(result)
    }
}
