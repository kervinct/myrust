use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write, Error};

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("Incoming connection from: {}", stream.peer_addr()?);  // 获取请求地址
    let mut buf = [0; 1024];  // 创建缓存
    loop {
        let bytes_read = stream.read(&mut buf)?;  // 读取数据
        if bytes_read == 0 { return Ok(()); }     // EOF，断开连接
        stream.write(&buf[..bytes_read])?;        // echo
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080")  // 绑定本地端口
        .expect("Could not bind");

    for stream in listener.incoming() {   // 迭代获取连接
        match stream {  // 模式匹配处理连接
            Err(e) => { eprintln!("failed: {}", e) }
            Ok(stream) => {
                thread::spawn(move || {   // 线程处理请求
                    handle_client(stream)
                        .unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }
}