#[derive(Debug)]
struct OurError(String);

impl std::error::Error for OurError {}

impl std::fmt::Display for OurError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn new(msg: &str) -> Box<dyn std::error::Error> {
    return Box::new(OurError(String::from(msg)));
}

#[allow(dead_code)]
pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;
