use std::env;

mod server;
mod client;
mod packet;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 0 { 
        println!("Please add an option to run. Client or Server");
        return Ok(())
    }
    match args[1].as_str() {
        "server" => {
            let mut server = server::Server::new();
            server.start();
        }
        "write_client" => {
            let mut client = client::write::Client::new();
            client.write_to_server();
        },
        "chat_client" => {
            let mut client = client::chat::Client::new();
            client.listener();
        }
        _ => {
            println!("Invalid option");
        }
    }

    Ok(())
}
