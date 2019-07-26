use futures::{future, Future};
use actix::*;
use std::collections::HashMap;

mod actors;

use self::actors::{
    rick::{Id, Rick, Count, AddrUpdate}
};

fn main() {
    let sys = System::new("test");

    let rick = Rick::new().start();
    let morty = Rick::new().start();

    let mut new_entries = HashMap::new();
    new_entries.insert(
        0,
        rick.clone(),
    );
    new_entries.insert(
        1,
        morty.clone(),
    );

    let upd = AddrUpdate {
        Addrs: new_entries,
    };
    let re = rick.send(upd);
    Arbiter::spawn(re.then(|re| {
        match re {
            Ok(result) => println!("SUM: {}", result),
            _ => println!("Something wrong"),
        }

        System::current().stop();
        future::result(Ok(()))
    }));


    let msg = Count {
        msg: String::from("hi there"),
    };
    let res = rick.send(msg);  // <- send message and get future for result

    Arbiter::spawn(res.then(|res| {
        match res {
            Ok(result) => println!("SUM: {}", result),
            _ => println!("Something wrong"),
        }

        System::current().stop();
        future::result(Ok(()))
    }));

    sys.run();
}
