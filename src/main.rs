mod context;
mod workers;

use std::{fs, io};
use std::time::Instant;
use async_std::task;
use clap::Parser;
use crate::context::Context;
use crate::workers::Worker;
use rand::seq::SliceRandom;

#[derive(Parser)]
struct Args {
    ///Paths to testdata
    #[arg(short, long)]
    paths: Vec<String>,
    ///Number of workers
    #[arg(short, long)]
    workers: usize,
    ///Use raw blobs instead of tarballs
    #[arg(short, long,default_value_t = true)]
    blob: bool,
}

#[async_std::main]
async fn main() -> io::Result<()> {
    let args: Args = Args::parse();
    let start = Instant::now();
    let ctx = Context::new();
    let mut file_count = 0;
    let mut record_count = 0;
    let mut odd_count = 0;
    let mut paths = Vec::new();
    for path in args.paths{
        let list = match fs::read_dir(path) {
            Ok(l) => {l}
            Err(e) => {return Err(e)}
        };
        for file in list{
            file_count += 1;
            let file = file.unwrap().path().to_str().unwrap().to_string();
            paths.push(file);
        }
    }
    let random_paths:Vec<String> = paths.choose_multiple(&mut rand::thread_rng(),paths.len()).cloned().collect();
    for path in random_paths{
        println!("{}",path);
        ctx.path_sender.send(path).await.unwrap();
    }


    for _ in 0..args.workers{
        let worker = Worker::new(ctx.clone());
        if args.blob{
            task::spawn(async move {
                worker.process_blobs().await
            });
        }else{
            task::spawn(async move {
                worker.process_archives().await
            });
        }

    }

    loop{
        let result = ctx.result_receiver.recv().await.unwrap();
        record_count += result.records_count;
        odd_count += result.odd_count;
        file_count -= 1;
        if file_count <= 0 {
            break
        }
    }

    println!("{} records processed in {}s. {:.0} records/s",record_count,start.elapsed().as_secs(),
             (record_count*1000000) as f64/start.elapsed().as_micros() as f64);
    println!("RESULT:");
    println!("odd count = {}",odd_count);
    println!("even count = {}",record_count - odd_count);

    Ok(())
}