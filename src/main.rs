use std::fs::File;
use std::io;
use std::io::Read;
use std::time::Instant;
use flate2::read::GzDecoder;
use tar::Archive;
use serde_json::{Result,Value};
use clap::Parser;

#[derive(Parser,Debug)]
struct Args {
    ///Path to archive with testdata to process
    #[arg(short, long)]
    path: String,
}

fn main() -> io::Result<()> {
    let args: Args = Args::parse();
    let tar_gz = File::open(args.path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    let mut counter = 0u64;
    let mut start = Instant::now();
    archive.entries().unwrap().for_each(|x|{
        let mut str: String = "".to_string();
        x.unwrap().read_to_string(&mut str);
        match serde_json::from_str::<Value>(str.as_str()){
            Ok(l) => {
                counter += 1;
                //println!("{}",l["user_id"])
            }
            Err(e) => {
               // println!("error: {}",e)
            }
        }


    });
    println!("{} records processed in {}s. {:.0} records/s",counter,start.elapsed().as_secs(),(counter*1000000) as f64/start.elapsed().as_micros() as f64);

    Ok(())
}