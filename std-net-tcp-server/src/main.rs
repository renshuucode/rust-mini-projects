use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0u8; 512];
    match stream.read(&mut buffer) {
        Ok(n) if n > 0 => {
            println!("收到客户端数据: {}", String::from_utf8_lossy(&buffer[..n]));

            let response = b"Hello from rust server!\n";
            stream.write_all(response).unwrap();
        },
        Ok(_) => println!("客户端主动关闭连接"),
        Err(e) => println!("读取时出错: {}", e),
    }
    
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("服务器启动，监听 127.0.0.1:7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("有新客户端连入: {}", stream.peer_addr()?);
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("连接失败: {}", e);
            }
        }
    }
    Ok(())
}