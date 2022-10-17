use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::num::ParseIntError;
use flate2::read::ZlibDecoder;
use tar::Archive;
use super::context::Context;
use serde_json::Value;
use crate::context::Aggregate;

pub struct Worker{
    ctx: Context
}

impl Worker {
    pub fn new(ctx: Context) -> Self{
        Self{
            ctx
        }
    }
    pub async fn process_blobs(self) {
        loop {
            let filepath = self.ctx.path_receiver.recv().await.unwrap();

            let file = match File::open(filepath) {
                Ok(file) => {file}
                Err(e) => {
                    println!("error: {}",e);
                    continue
                }
            };
            let mut result = Aggregate::new();
            let mut reader = BufReader::new(file);
            for line in reader.lines(){
                let str;
                match line{
                    Ok(l) => {
                        str = l;
                    }
                    Err(e) => {
                        println!("{}",e);
                        continue;
                    }
                }
                match serde_json::from_str::<Value>(str.as_str()){
                    Ok(l) => {
                        let id = match l["ts"].as_u64() {
                            None => { continue }
                            Some(id) => {id}
                        };
                        result.records_count += 1;
                        if id%2 != 0 {
                            result.odd_count += 1;
                        }
                    }
                    Err(e) => {

                    }
                }
            }


            self.ctx.result_sender.send(result).await.unwrap();
        }
    }
    pub async fn process_archives(self) {
        loop {
            let filepath = self.ctx.path_receiver.recv().await.unwrap();

            let file = match File::open(filepath) {
                Ok(file) => {file}
                Err(e) => {
                    println!("error: {}",e);
                    continue
                }
            };
            let tar = ZlibDecoder::new(file);
            let mut archive = Archive::new(tar);
            let mut counter = 0u64;

            archive.entries().unwrap().for_each(|x|{
                let mut str: String = "".to_string();
                match x.unwrap().read_to_string(&mut str){
                    Ok(_) => { counter += 1}
                    Err(_) => {}
                }
                // match serde_json::from_str::<Value>(str.as_str()){
                //     Ok(l) => {
                //         counter += 1;
                //     }
                //     Err(e) => {
                //
                //     }
                // }
            });
            self.ctx.result_sender.send(Aggregate{
                records_count: counter,
                odd_count: counter
            }).await.unwrap();
        }
    }
}