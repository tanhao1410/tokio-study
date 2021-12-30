use mini_redis::{client,Result};


#[tokio::main]
async fn main()->Result<()>{
    // let say_world = say_world();
    // println!("hello");
    // say_world.await;

    //打开一个连接
    let mut client = client::connect("127.0.0.1:6379").await?;
    //set
    client.set("hello","world".into()).await?;
    let result = client.get("hello").await?;
    println!("got value from server;result={:?}",result);
    Ok(())
}


async fn say_world(){
    println!("world!")
}
