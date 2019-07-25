use may::coroutine::JoinHandle;
use may::net::TcpListener;
use tungstenite::server::accept;

pub(crate) fn run_websocket_server() -> JoinHandle<()>  {
    let handler = go!(move || {
        let listener = TcpListener::bind(("0.0.0.0", 8080)).unwrap();
        for stream in listener.incoming() {
            go!(move || -> () {
                let mut websocket = accept(stream.unwrap()).unwrap();

                loop {
                    let msg = websocket.read_message().unwrap();

                    // Just echo back everything that the client sent to us
                    if msg.is_binary() || msg.is_text() {
                        websocket.write_message(msg).unwrap();
                    }
                }
            });
        }
    });

    handler

    // println!("Websocket server running on ws://0.0.0.0:8080");
    // handler.join().unwrap();
}
