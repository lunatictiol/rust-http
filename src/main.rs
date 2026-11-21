fn main() {
    let addr = String::from("198.8.0.0");
    let server = Server::new(addr);
    server.start();



    println!("Hello, world!");
}

struct Server {
    addr: String,
}


impl Server {
    fn new(addr: String) -> Self {
        Server { addr }
    }

    fn start(&self) {
        println!("Server is running on {}", self.addr);
    }
}