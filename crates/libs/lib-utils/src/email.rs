use regex::Regex;

pub fn is_valid_email(email: &str) -> bool {
    let re = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
    re.is_match(email)
}
