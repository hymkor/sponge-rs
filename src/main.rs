fn sponge(original: &str) -> std::io::Result<()> {
    let tmp_name = format!("{}.sponge", original);
    {
        let mut r = std::io::stdin();
        let mut w = std::fs::File::create(&tmp_name)?;
        std::io::copy(&mut r, &mut w)?;
    }
    if std::path::Path::new(original).try_exists()? {
        let backup = format!("{}~", original);
        std::fs::rename(original, backup)?;
    }
    std::fs::rename(tmp_name, original)?;
    return Ok(());
}

fn mains() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let original = match args.next() {
        Some(original) => original,
        None => return Err(Box::from("filename is not specified")),
    };
    sponge(&original)?;
    Ok(())
}

fn main() {
    if let Err(err) = mains() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
