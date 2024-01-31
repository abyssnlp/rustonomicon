use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use bytes::Bytes;
use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

// In case of contention, the mutex is held across the lifecycle of the task and blocks
// We can shard the mutex as we have the key and can decide which shard the key/value goes to

type ShardedDB = Arc<Vec<Mutex<HashMap<String, Vec<u8>>>>>;

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

fn new_sharded_db(num_shards: usize) -> ShardedDB {
    let mut db = Vec::with_capacity(num_shards);
    for _ in 0..num_shards {
        db.push(Mutex::new(HashMap::new()));
    }
    Arc::new(db)
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:6379").await.unwrap();

    println!("Listening...");

    // To search in case of a sharded db
    // let shard = db[hash(key) % db.len()].lock().unwrap();
    // shard.insert(key, value);

    let db: Db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db = db.clone();
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};

    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }

            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented: {:?}", cmd),
        };
        connection.write_frame(&response).await.unwrap();
    }
}
