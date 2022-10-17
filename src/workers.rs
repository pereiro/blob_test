use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use flate2::read::ZlibDecoder;
use tar::Archive;
use super::context::Context;

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
            let mut counter = 0u64;
            let mut reader = BufReader::new(file);
            for line in reader.lines(){
                match line{
                    Ok(l) => {
                        counter += 1;
                    }
                    Err(e) => { println!("{}",e)}
                }
                // match serde_json::from_str::<Value>(str.as_str()){
                //     Ok(l) => {
                //         counter += 1;
                //     }
                //     Err(e) => {
                //
                //     }
                // }
            }


            self.ctx.result_sender.send(counter).await.unwrap();
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
            self.ctx.result_sender.send(counter).await.unwrap();
        }
    }
}