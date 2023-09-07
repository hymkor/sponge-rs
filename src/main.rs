use std::io::BufRead;
use std::io::Write;

fn mains(args: std::env::Args) -> Result<(),Box<dyn std::error::Error>> {
    if let Some(original) = args.skip(1).next() {
        let tmp_name = original.clone() + ".sponge";
        {
            let fd = std::fs::File::create(&tmp_name)?;
            let mut bw = std::io::BufWriter::new(fd);
            let reader = std::io::BufReader::new(std::io::stdin());
            for line in reader.lines() {
                bw.write(&line?.as_bytes())?;
                bw.write(&[b'\n'])?;
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
