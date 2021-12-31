use tokio::io;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;
    loop {
        let (mut socket, _attr) = listener.accept().await?;

        tokio::spawn(async move {
            //可以将实现了AsyncReader与AsyncWrite的进行拆分
            // let (mut reader, mut writer) = io::split(socket);
            // match io::copy(&mut reader, &mut writer).await {
            //     Ok(v) => println!("copy success:{}", v),
            //     Err(e) => eprintln!("failed to copy!{:?}", e)
            // }
            let mut buf = vec![0; 128];
            match socket.read(&mut buf).await {
                //当连接关闭后，会返回OK ,读到的数据是0
                Ok(0) => return,
                Ok(n) => {
                    if socket.write_all(&buf[..n]).await.is_err() {
                        return;
                    }
                }
                Err(_) => return,
            }
        });
    }
}