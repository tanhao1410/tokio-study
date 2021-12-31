#[tokio::main]
async fn main()->io::Result<()>{
    println!("hello world!");
    //async_write_demo1().await?;
    //async_reader_demo1().await?;
    //io_copy_demo().await?;

    echo_demo().await?;


    Ok(())
}


async fn echo_demo()->io::Result<()>{
    let stream = TcpStream::connect("127.0.0.1:6142").await?;
    let (mut read,mut write) = io::split(stream);

    tokio::spawn(async move{
        write.write_all(b"hello world\n").await?;
        write.write_all(b"echo \n").await?;

        Ok::<_,io::Error>(())
    });

    let mut buf = vec![0;128];
    loop {
        let n = read.read(&mut buf).await?;

        if n == 0{
            break;
        }
        println!("Got:{:?}",&buf[..n]);
    }
    Ok(())
}


async fn io_copy_demo()->io::Result<()>{

    let mut reader:&[u8] = b"hello world";
    let mut file = File::create("hello.txt").await?;
    io::copy(&mut reader,&mut file).await?;
    Ok(())
}

async fn async_reader_demo1()->io::Result<()>{
    let mut file = File::open("foo.txt").await?;

    let mut buf = [0u8;10];

    let n = file.read(&mut buf).await?;

    println!("read :{:?}",buf);

    Ok(())
}

/// AsyncWriteExt demo
use tokio::io::{self, AsyncWriteExt, AsyncReadExt};
use tokio::fs::File;
use tokio::net::TcpStream;

async fn async_write_demo1()->io::Result<()>{
    let mut file = File::create("foo.txt").await?;

    let n = file.write(b"some bytes").await?;

    println!("写了{} bytes",n);
    Ok(())
}