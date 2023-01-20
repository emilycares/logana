use lazy_static::lazy_static;
use regex::Regex;

/// Removes color
#[must_use]
pub fn strip_color(text: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new("\\x1B\\[(?:;?[0-9]{1,3})+[mGK]")
            .expect("Unbale to create regex to strip color");
    }
    RE.replace_all(text, String::new()).to_string()
}
