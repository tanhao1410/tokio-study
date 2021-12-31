use mini_redis::client;
use bytes::Bytes;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

//在给它传递一个命令的时候，同时包含一个结果发送者，当处理好后，需要将结果发送过来
type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}

#[tokio::main]
async fn main() {
    let mut client = client::connect("127.0.0.1:6379").await.unwrap();

    //创建一个多生产者，单消费者channel
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    let t1 = tokio::spawn(async move {
        let (sender, reciver) = oneshot::channel();
        let cmd = Command::Get {
            key: "hello".to_string(),
            resp: sender,
        };
        tx.send(cmd).await.unwrap();
        let result = reciver.await.unwrap().unwrap().unwrap();
        println!("got :{}", String::from_utf8(result.to_vec()).unwrap());
    });


    let t2 = tokio::spawn(async move {
        let (sender, reciver) = oneshot::channel();
        let cmd = Command::Set {
            key: "hello".to_string(),
            val: "world".into(),
            resp: sender,
        };
        tx2.send(cmd).await.unwrap();
        reciver.await;
    });

    let num = 100i32;

    let res = (1..=num / 2).filter(|&n| num % n == 0).sum::<i32>();
    (1..)
        .take_while(|&n| n * n > num)
        .filter(|&n|num % n == 0 && num != n)
        .map(|n| n + num / n)
        .sum::<i32>()

    let manage = tokio::spawn(async move {
        while let Some(v) = rx.recv().await {
            match v {
                Command::Get { key, resp } => {
                    let res = client.get(key.as_str()).await;
                    resp.send(res);
                }
                Command::Set { key, val: value, resp } => {
                    let res = client.set(&key, value).await;
                    resp.send(res);
                }
            }
        }
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manage.await.unwrap();
}