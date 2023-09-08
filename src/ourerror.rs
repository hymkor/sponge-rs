#[derive(Debug)]
struct OurError {
    msg: String,
}

impl std::error::Error for OurError {}

impl std::fmt::Display for OurError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{}",self.msg)
    }
}

pub fn new(msg: &str) -> Box<dyn std::error::Error> {
    return Box::new(OurError{ msg:String::from(msg) })
}
