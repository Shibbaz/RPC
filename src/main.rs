use may::net::TcpStream;
use std::time::Duration;
use may::coroutine;
#[macro_use]
extern crate may;
#[may_rpc::service]
trait Hello {
    fn hello(&self, name: String) -> String;
}

#[derive(may_rpc::Server)]
#[service(Hello)]
struct HelloServer;

impl Hello for HelloServer {
    fn hello(&self, name: String) -> String {
        format!("{}!", name)
    }
}

fn main() {
    use may_rpc::TcpServer;
    let addr = "127.0.0.1:11000";
    let _server = HelloServer.start(addr).unwrap();
    go!(move || {
        let (tx1, rx1) = may::sync::mpsc::channel();
        let (tx2, rx2) = may::sync::mpsc::channel();
        tx1.send("tx1: Out").unwrap();
        coroutine::sleep(Duration::from_millis(100));
        go!(move || {

            tx2.send("tx2: In").unwrap();
            coroutine::sleep(Duration::from_millis(100));

            let (tx3, rx3) = may::sync::mpsc::channel();
            tx3.send("tx3: In").unwrap();

            let stream = TcpStream::connect(addr).unwrap();
            let client = HelloClient::new(stream).unwrap();
            may::loop_select!(
                v2 = rx2.recv() => println!("{}", client.hello(v2.unwrap().to_string()).unwrap()),
                _ = coroutine::sleep(Duration::from_millis(1000)) => {},
                v3 = rx3.recv() => println!("{}", client.hello(v3.unwrap().to_string()).unwrap())
            );

        });
        
        let stream = TcpStream::connect(addr).unwrap();
        let client = HelloClient::new(stream).unwrap();
        may::loop_select!(
            v1 = rx1.recv() => println!("{}", client.hello(v1.unwrap().to_string()).unwrap())
        );
    });
    std::thread::sleep(Duration::from_secs(1));
    
}