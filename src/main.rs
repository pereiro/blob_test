mod context;
mod workers;

use std::{fs, io};
use std::io::Read;
use std::path::PathBuf;
use std::time::Instant;
use async_std::channel::RecvError;
use async_std::task;
use tar::Archive;
use serde_json::{Result,Value};
use clap::Parser;
use crate::context::Context;
use crate::workers::Worker;

#[derive(Parser)]
struct Args {
    ///Paths to testdata
    #[arg(short, long)]
    paths: Vec<String>,
    ///Number of workers
    #[arg(short, long)]
    workers: usize
}

#[async_std::main]
async fn main() -> io::Result<()> {
    let args: Args = Args::parse();
    let mut start = Instant::now();
    let ctx = Context::new();
    let mut file_count = 0;
    let mut record_count = 0;
    for path in args.paths{
        let list = match fs::read_dir(path) {
            Ok(l) => {l}
            Err(e) => {return Err(e)}
        };
        for file in list{
            file_count += 1;
            let file = file.unwrap().path().to_str().unwrap().to_string();
            println!("{}",file);
            ctx.path_sender.send(file).await.unwrap();
        }
    }

    for _ in 0..args.workers{
        let worker = Worker::new(ctx.clone());
        task::spawn(async move {
            worker.start().await
        });
    }

    loop{
        let result = ctx.result_receiver.recv().await.unwrap();
        record_count += result;
        file_count -= 1;
        if file_count <= 0 {
            break
        }
    }

    println!("{} records processed in {}s. {:.0} records/s",record_count,start.elapsed().as_secs(),
             (record_count*1000000) as f64/start.elapsed().as_micros() as f64);

    Ok(())
}