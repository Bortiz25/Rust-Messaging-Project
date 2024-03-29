//use std::fmt::Error;
use std::process;
use std::io;
use tokio::runtime;

use messaging_project::{ ChatCommand, DeleteCommand, ListCommand, MessageCommand, UserCommand };

fn main() {
    let mut buf = String::new();
    let _stream = io::stdin().read_line(&mut buf);
    buf.pop();
    let new_args: Vec<&str> = buf.split("\"").collect();
    let mut args: Vec<&str> = new_args[0].split(" ").collect();
    if new_args.len() > 1 {
        args.push(new_args[1]);
    }
    dbg!(&args);
    //Ok(());

    let rt = runtime::Runtime::new().unwrap();
    //let check = &the_args[0];
    if &args[0] == &"send" {
        let config = MessageCommand::build(args).unwrap_or_else(|err| {
            println!("Problem parsing arguments: {err}");
            process::exit(1);
        });
        println!("Successful constructed, {:?}", config);
    } else if &args[0] == &"chat" {
        let config = ChatCommand::build(args).unwrap_or_else(|err| {
            println!("Problem parsing arguments: {err}");
            process::exit(1);
        });
        println!("Successful constructed, {:?}", config)
    } else if &args[0] == &"user" {
        let config = rt.block_on(UserCommand::build(args)).unwrap();
        println!("Successful constructed, {:?}", config)
    } else if &args[0] == &"delete" {
        let config = DeleteCommand::build(args).unwrap_or_else(|err| {
            println!("Problem parsing arguments: {err}");
            process::exit(1);
        });
        println!("Successful constructed, {:?}", config)
    } else if &args[0] == &"list" {
        let config = ListCommand::build(args).unwrap_or_else(|err| {
            println!("Problem parsing arguments: {err}");
            process::exit(1);
        });
        println!("Successful constructed, {:?}", config)
    }
}
