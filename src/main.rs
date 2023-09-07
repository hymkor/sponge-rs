use thiserror::Error;

#[derive(Debug, Error)]
enum OurError {
    #[error("filename is not specified")]
    NoFileName,
}

fn mains(args: std::env::Args) -> Result<(),Box<dyn std::error::Error>> {
    let original = match args.skip(1).next() {
        Some(original) => original,
        None => { return Err(Box::new(OurError::NoFileName)) },
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
