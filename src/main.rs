#[derive(Debug)]
struct OurError {
    msg: String,
}

impl std::error::Error for OurError {}

impl std::fmt::Display for OurError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.msg)
    }
}

fn our_error(msg: &str) -> Box<dyn std::error::Error> {
    return Box::new(OurError{ msg:String::from(msg) })
}

fn sponge(original: String) -> std::io::Result<()> {
    let tmp_name = format!("{}.sponge",&original);
    {
        let mut r = std::io::stdin();
        let mut w = std::fs::File::create(&tmp_name)?;
        std::io::copy(&mut r,&mut w)?;
    }
    if std::path::Path::new(&original).try_exists()? {
        let backup = format!("{}~",&original);
        std::fs::rename(&original,backup)?;
    }
    std::fs::rename(tmp_name,original)?;
    return Ok(())
}

fn mains() -> Result<(),Box<dyn std::error::Error>> {
    let mut args = std::env::args();
    if let None = args.next() {
        return Err(our_error("filename is not specified") )
    }
    let original = match args.next() {
        Some(original) => original,
        None => return Err(our_error("filename is not specified") )
    };
    if let Some(_) = args.next() {
        return Err(our_error("too many filenames") )
    }
    sponge(original)?;
    return Ok(())
}

fn main(){
    if let Err(err) = mains() {
        eprintln!("{}",err);
        std::process::exit(1);
    }
}
