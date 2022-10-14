use std::fs::File;
use std::io::Read;
use flate2::read::GzDecoder;
use serde_json::Value;
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
    pub async fn start(self) {
        loop {
            let filepath = self.ctx.path_receiver.recv().await.unwrap();

            let tar_gz = match File::open(filepath) {
                Ok(file) => {file}
                Err(e) => {
                    println!("error: {}",e);
                    continue
                }
            };
            let tar = GzDecoder::new(tar_gz);
            let mut archive = Archive::new(tar);
            let mut counter = 0u64;

            archive.entries().unwrap().for_each(|x|{
                let mut str: String = "".to_string();
                match x.unwrap().read_to_string(&mut str){
                    Ok(x) => { counter += 1}
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