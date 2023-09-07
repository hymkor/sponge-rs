fn mains(args: std::env::Args) -> std::io::Result<()> {
    let original = match args.skip(1).next() {
        Some(original) => original,
        None => {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "file is not specified"))
        }
    };
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

fn main(){
    if let Err(err) = mains(std::env::args()) {
        eprintln!("{}",err);
        std::process::exit(1);
    }
}
