use rand::Rng;
use actix::{Actor, Message, Handler, Context, Addr};
use std::collections::HashMap;

type Value = u64;
pub type Id = u16;

pub struct MyFriend {
    addr: Addr<Rick>
}

pub struct Rick {
    pub id: Id,
    pub addr_book: HashMap<Id, Addr<Rick>>
}

impl Rick {
    pub fn new() -> Self {
        Self {
            id: rand::thread_rng().gen_range(0,100),
            addr_book: HashMap::new(),
        }
    }
}

impl Actor for Rick {
    type Context = Context<Self>;
}

pub struct Count{
    pub msg: String
}

impl Message for Count {
    type Result = Value;
}

impl Handler<Count> for Rick {
    type Result = Value;

    fn handle(&mut self, Count: Count, _: &mut Context<Self>) -> Self::Result {

        let mut rng = rand::thread_rng();
        let draw: u64 = rng.gen_range(0, 100);

        draw
    }
}

// Update address book
pub struct AddrUpdate {
    pub Addrs: HashMap<Id, Addr<Rick>>,
}

impl Message for AddrUpdate {
    type Result = Value;
}

impl Handler<AddrUpdate> for Rick {
    type Result = Value;

    fn handle(&mut self, AddrUpdate: AddrUpdate, _: &mut Context<Self>) -> Self::Result {
        
        self.addr_book = AddrUpdate.Addrs;
        println!("Well this worked");

        0
    }
}


