use std::io::Read;
use std::io::Write;

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
        let mut buffer = [0u8;1024];
        let mut w = std::fs::File::create(&tmp_name)?;
        loop{
            let n = r.read(&mut buffer)?;
            if n <= 0 {
                break;
            }
            w.write_all(&buffer[0..n])?;
        }
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
