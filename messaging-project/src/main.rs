//use std::fmt::Error;
use std::process;
use std::io;
use messaging_project::CreateUserCommand;
use tokio::runtime;

use messaging_project::{
    ChatCommand,
    DeleteCommand,
    ListCommand,
    MessageCommand,
    UserCommand,
    LoginCommand,
    ret_chats,
};

fn main() {
    let mut buf = String::new();
    let mut tok: String = String::new();

    loop {
        buf = String::new();
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
            let config = rt.block_on(MessageCommand::build(args, &tok)).unwrap();
            println!("Successful constructed, {:?}", config);
        } else if &args[0] == &"messages" {
            let config = rt.block_on(ChatCommand::build(args, &tok)).unwrap();
            println!("Successful constructed, {:?}", config);
        } else if &args[0] == &"chats" {
            let config = rt.block_on(ret_chats(args, &tok)).unwrap();
            println!("Successful constructed, {:?}", config);
        } else if &args[0] == &"user" {
            let config = rt.block_on(UserCommand::build(args)).unwrap();
            println!("Successful constructed, {:?}", config);
        } else if &args[0] == &"delete" {
            let config = DeleteCommand::build(args).unwrap_or_else(|err| {
                println!("Problem parsing arguments: {err}");
                process::exit(1);
            });
            println!("Successful constructed, {:?}", config);
        } else if &args[0] == &"list" {
            let config = ListCommand::build(args).unwrap_or_else(|err| {
                println!("Problem parsing arguments: {err}");
                process::exit(1);
            });
            println!("Successful constructed, {:?}", config);
        } else if &args[0] == &"username" {
            let config = rt.block_on(LoginCommand::build(args)).unwrap();
            println!("{:?}", &config);
            tok = config;
            println!("Successfully signed in!");
        } else if &args[0] == &"createuser" {
            let config = rt.block_on(CreateUserCommand::build(args)).unwrap();
            println!("Create User Status, {:?}", config);
            println!("202 : successful user creation");
        } else if &args[0] == &":q" {
            break;
        } else {
            println!("Perform a -h or -help command to view valid inputs.");
            break;
        }
    }
}
