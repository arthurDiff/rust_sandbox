use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

pub fn run() {
    let first_t = thread::spawn(|| {
        for i in 1..10 {
            println!("{} call to string", i);
        }
    });

    for i in 1..10 {
        println!("This is from outside {}", i);
    }

    let v = vec![1, 23, 4, 5];
    let move_thread = thread::spawn(move || {
        println!("this is moved: {:?}", v);
    });
    move_thread.join().unwrap();

    first_t.join().unwrap();

    playing_w_channel();

    playing_w_shared();
}

fn playing_w_channel() {
    // Trasmitter, Receiver =  Multiple Producer|Single Consumer => MPSC
    let (tx, rx) = mpsc::channel();
    let (tx1, tx2) = (tx.clone(), tx.clone());

    thread::scope(|s| {
        s.spawn(move || {
            let val = vec![
                String::from("HELLLO FROM TREAD"),
                String::from("ANother text"),
                String::from("OneMore"),
            ];

            for val_str in val {
                if tx.send(val_str).is_ok() {
                    println!("datasent");
                };
                thread::sleep(Duration::from_secs(1));
            }
        });

        s.spawn(move || {
            let val = vec![
                String::from("t1HELLLO FROM TREAD"),
                String::from("t1ANother text"),
                String::from("t1OneMore"),
            ];

            for val_str in val {
                if tx1.send(val_str).is_ok() {
                    println!("datasent");
                };
                thread::sleep(Duration::from_secs(1));
            }
        });

        s.spawn(move || {
            let val = vec![
                String::from("t2 HELLLO FROM TREAD"),
                String::from("t2 ANother text"),
                String::from("t2 OneMore"),
            ];

            for val_str in val {
                if tx2.send(val_str).is_ok() {
                    println!("datasent");
                };
                thread::sleep(Duration::from_secs(1));
            }
        });
        s.spawn(move || {
            //rx.try_recv() will not block thread;
            // let received_msg = rx.recv().unwrap_or(String::from("GOT NOTTHING"));
            for received in rx {
                println!("{}", received);
            }
        });
    });
}
//RefCell-RC <-> Mutext-Arc
fn playing_w_shared() {
    //mutual exclusion -> Arc = Atomic Reference Counted
    //for simple types just use atomic value
    let counter = Arc::new(Mutex::new(0));
    let mut handler = Vec::new();

    for _ in 0..10 {
        let amtx = Arc::clone(&counter);
        let t = thread::spawn(move || {
            let mut c = amtx.lock().unwrap();
            *c += 1;
        });
        handler.push(t);
    }

    for trs in handler {
        trs.join().unwrap();
    }
    println!("Result from arc mutex {}", counter.lock().unwrap());
}
