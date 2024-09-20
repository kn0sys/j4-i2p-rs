```rust
use std::{thread,fs::File, io::{self, BufRead}, path::Path};
use j4i2prs::router_wrapper as rw;
use j4i2prs::tunnel_control as tc;
use j4i2prs::error as e;

use std::sync::mpsc::{
    Receiver,
    Sender,
};

struct Listener {
    is_running: bool,
    run_tx: Sender<bool>,
    run_rx: Receiver<bool>,
}

impl Default for Listener {
    fn default() -> Self {
        let is_running = false;
        let (run_tx, run_rx) = std::sync::mpsc::channel();
        Listener {
            is_running,
            run_tx,
            run_rx,
        }
    }
}

/// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// fake app, replace println! with logging
fn main() -> Result<(), e::J4I2PRSError> {
    //env_logger::init(); 
    let r = rw::Wrapper::create_router()?;
    let mut l: Listener = Default::default();
    let run_tx = l.run_tx.clone();
    let run_handle = thread::spawn(move || {
        println!("run thread started");
        run_tx.send(true).unwrap_or_else(|_| println!("failed to run router"));
    });
    // let shutdown_handle = thread::spawn(move || {
    //    thread::sleep(std::time::Duration::from_secs(60));
    //    shutdown_tx.send(true).unwrap_or_else(|_| println!("failed to shutdown router"))
    // });
    // run the main thread forever unless we get a router shutdown signal
    let listener_handle = thread::spawn(move || {
        // ctrl+c signal handler
        //let _ = ctrlc::set_handler(move || {
        //    println!("received Ctrl+C!");
        //    std::process::exit(0);
        //});
        std::thread::sleep(std::time::Duration::from_secs(10));
        loop {
            if let Ok(run) = l.run_rx.try_recv() {
                if run {
                    println!("starting router");
                    r.invoke_router(rw::METHOD_RUN).unwrap_or_else(|_| println!("failed to run router"));
                }
            }
            if !l.is_running {
                let is_router_on = r.is_running().unwrap_or_default();
                if !is_router_on {
                    println!("router is not on");
                }
                std::thread::sleep(std::time::Duration::from_secs(60));
                if is_router_on {
                    // check router config
                    if let Ok(lines) = read_lines("./router.config") {
                        for line in lines.map_while(Result::ok) {
                            if line.contains("i2np.udp.port") {
                                let port = line.split("=").collect::<Vec<&str>>()[1];
                                println!("router is running on external port = {}", port);
                                println!("open this port for better connectivity");
                                println!("this port was randomly assigned, keep it private");
                                l.is_running = true;
                                // start the http proxy
                                let http_proxy: tc::Tunnel = tc::Tunnel::new("127.0.0.1".to_string(), 4455, tc::TunnelType::Http)
                                    .unwrap_or_default();
                                let _ = http_proxy.start(None);
                                println!("http proxy on port {}", http_proxy.get_port());
                                // start the tunnel
                                let app_tunnel: tc::Tunnel = tc::Tunnel::new("127.0.0.1".to_string(), 3000, tc::TunnelType::Server)
                                    .unwrap_or_default();
                                println!("destination: {}", app_tunnel.get_destination());
                                let _ = app_tunnel.start(None);
                            }
                        }
                    }
                }
            }
        }
    });
    println!("waiting for work...");
    let _ = run_handle.join();
    let _ = listener_handle.join();
    Ok(())
}
```
