// use mini_redis::{Result};
use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

use std::collections::HashMap;
use mini_redis::Command::{self, Get, Set};
use std::sync::{Arc, Mutex};

use bytes::Bytes;

// 采用一个多个线程都可以访问到的db
type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {

    //tokio spawn example
    // let handle = tokio::spawn(async {
    //     "hello world"
    // });
    //
    // let result = handle.await.unwrap();
    // println!("{}",result);

    //绑定端口
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _addr) = listener.accept().await.unwrap();
        //process(socket).await;
        //let db = db.clone();
        let db = Arc::clone(&db);
        //采用并发的方式
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

/// 处理客户端请求信息
async fn process(socket: TcpStream, db: Db) {

    //let mut db:HashMap<String,Vec<u8>> = HashMap::new();

    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            cmd => panic!("unimplemented:{:?}", cmd)
        };
        connection.write_frame(&response).await.unwrap();
    }

    // if let Some(frame) = connection.read_frame().await.unwrap(){
    //     println!("GOT:{:?}",frame);
    //
    //     //发送信息
    //     let response = Frame::Error("unimplemented".to_string());
    //     connection.write_frame(&response).await.unwrap();
    // }
}