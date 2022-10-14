use std::path::PathBuf;
use async_std::channel::{bounded, unbounded};
use async_std::channel::{Receiver, Sender};

#[derive(Clone)]
pub struct Context{
    pub path_sender: Sender<String>,
    pub path_receiver: Receiver<String>,
    pub result_sender: Sender<u64>,
    pub result_receiver: Receiver<u64>,
}

impl Context {
    pub fn new() -> Self{
        let (path_sender,path_receiver) = unbounded();
        let (result_sender,result_receiver) = unbounded();
        Self{
            path_sender,
            path_receiver,
            result_sender,
            result_receiver,
        }
    }
}