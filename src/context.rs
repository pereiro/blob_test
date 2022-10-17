use async_std::channel::{unbounded};
use async_std::channel::{Receiver, Sender};

pub struct Aggregate{
    pub records_count: u64,
    pub odd_count: u64,
}

impl Aggregate {
    pub fn new()->Self{
        Aggregate{
            records_count: 0,
            odd_count: 0
        }
    }
}

#[derive(Clone)]
pub struct Context{
    pub path_sender: Sender<String>,
    pub path_receiver: Receiver<String>,
    pub result_sender: Sender<Aggregate>,
    pub result_receiver: Receiver<Aggregate>,
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