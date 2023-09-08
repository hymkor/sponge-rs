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

fn mains() -> Option<String> {
    let mut args = std::env::args();
    if let None = args.next() {
        return Some(String::from("filename is not specified"))
    }
    let original = match args.next() {
        Some(original) => original,
        None => { return Some(String::from("filename is not specified")) }
    };
    if let Some(_) = args.next() {
        return Some(String::from("too many filenames"))
    }
    if let Err(err) = sponge(original) {
        return Some(format!("{}",err))
    }
    return None
}

fn main(){
    if let Some(msg) = mains() {
        eprintln!("{}",msg);
        std::process::exit(1);
    }
}
