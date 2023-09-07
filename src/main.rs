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
    let tmp_name = original.clone() + ".sponge";
    {
        let stdin = std::io::stdin();
        let mut r = stdin.lock();
        let mut w = std::fs::File::create(&tmp_name)?;
        std::io::copy(&mut r,&mut w)?;
    }
    let original_path = std::path::Path::new(&original);
    if original_path.try_exists()? {
        let bak_name = original.clone() + "~";
        std::fs::rename(&original,bak_name)?;
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
