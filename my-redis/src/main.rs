// use mini_redis::{Result};
use tokio::net::{TcpListener,TcpStream};
use mini_redis::{Connection,Frame};

use std::collections::HashMap;
use mini_redis::Command::{self,Get,Set};

#[tokio::main]
async fn main(){

    //tokio spawn example
    // let handle = tokio::spawn(async {
    //     "hello world"
    // });
    //
    // let result = handle.await.unwrap();
    // println!("{}",result);

    //绑定端口
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (socket,_addr) = listener.accept().await.unwrap();
        //process(socket).await;
        //采用并发的方式
        tokio::spawn(async move{
            process(socket).await;
        });
    }

}

/// 处理客户端请求信息
async fn process(socket :TcpStream){

    let mut db:HashMap<String,Vec<u8>> = HashMap::new();

    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap(){
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()){
                    Frame::Bulk(value.clone().into())
                }else{
                    Frame::Null
                }
            }
            Set(cmd) => {
                db.insert(cmd.key().to_string(),cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            _=> unreachable!("unimplemented")
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