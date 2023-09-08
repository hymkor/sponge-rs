mod ourerror;

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
        return Err(ourerror::new("filename is not specified") )
    }
    let original = match args.next() {
        Some(original) => original,
        None => return Err(ourerror::new("filename is not specified") )
    };
    if let Some(_) = args.next() {
        return Err(ourerror::new("too many filenames") )
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
