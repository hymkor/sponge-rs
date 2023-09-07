use std::io::Write;
use std::io::Read;

fn mains(args: std::env::Args) -> Result<(),Box<dyn std::error::Error>> {
    if let Some(original) = args.skip(1).next() {
        let tmp_name = original.clone() + ".sponge";
        {
            let mut w = std::fs::File::create(&tmp_name)?;
            let stdin = std::io::stdin();
            let mut r = stdin.lock();
            let mut buffer = [0u8;1024];
            loop{
                let n = r.read(&mut buffer)?;
                if n <= 0 {
                    break;
                }
                w.write_all(&buffer[0..n])?;
            }
        }
        std::fs::rename(tmp_name,original)?;
    }
    return Ok(())
}

fn main(){
    if let Err(err) = mains(std::env::args()) {
        eprintln!("{}",err);
        std::process::exit(1);
    }
}
