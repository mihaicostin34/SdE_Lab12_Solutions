use tokio::{net::{TcpListener, TcpStream}, io::BufReader, io::{BufWriter, AsyncWriteExt}};
use tokio::io::AsyncBufReadExt;

#[tokio::main]
async fn main() {
    // TODO 1 - listen for connections using a TcpListener
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    // TODO 2 - spawn a new task for each accepted connection
    loop{
        let (socket, _) = listener.accept().await.unwrap();
        tokio::task::spawn(handle(socket));
    }
}

async fn handle(mut socket: TcpStream) {
    // TODO 3 - read the HTTP request and print the method and URL
    let add = socket.peer_addr().unwrap();
    let port = add.port();
    let ip = add.ip();
    let (read, write) = socket.split();
    let mut stream = BufReader::new(read);
    let mut i = 0;
    let mut path = "";
    loop{
        let mut line = String::new();
        stream.read_line(&mut line).await.unwrap();
        if i ==0{
            let words = line.split(" ").collect::<Vec<&str>>();
            let meth = words.get(0).unwrap();
            println!("Method: {meth}");
            path = words.get(1).unwrap();
            println!("Path : {path}");
            i+=1;
        } 
        println!("{:?}", line);
        if line.eq_ignore_ascii_case("\r\n") {
            break;
        }
    }
    // TODO 4 - print the remote IP and port
    println!("Socket with ip address {ip} at port {port}");
    
    // TODO 5 - send a response with the received URL (the browser should display the url)
    let mut ostream = BufWriter::new(write);
    let mut buf = String::from("HTTP/1.0 200 OK\r\nContent-type:text/html\r\n\r\n<html>
        <body>
        <p>Continut</p>
        </body>
        </html>\r\n");
    ostream.write_all(&mut buf.as_bytes()).await.unwrap();
    ostream.flush().await;
}
