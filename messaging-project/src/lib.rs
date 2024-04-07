use std::fmt::{ self, format };
use std::collections::HashMap;
use std::future::Future;
use reqwest::{ Client, Error, Response, StatusCode };
use serde::Deserialize;

async fn message_post_helper(username: &str, message: &str) -> Result<Response, &'static str> {
    let url: String = format!("http://localhost:8001/chats/{}/", username);
    let mut map = HashMap::new();
    map.insert("message", message);

    let client = reqwest::Client::new();
    let res = client.post(url).json(&map).send().await;
    let final_res = match res {
        Ok(r) => r,
        Err(_) => {
            return Err("Error: posting request");
        }
    };
    Ok(final_res)
}

//TODO: Fix type issues
async fn message_get_helper(username: &str) -> Result<Message, Error> {
    let url: String = format!("http://localhost:8001/chats/{}/messages", username);
    let get_res = reqwest::get(&url).await?;

    let final_result = get_res.json::<Message>().await?;
    Ok(final_result)
}

async fn user_post_helper(username: &str, password: &str) -> Result<StatusCode, &'static str> {
    let url: String = format!("http://localhost:8001/users/");
    let mut map = HashMap::new();
    map.insert("username", username);
    map.insert("password", password);

    let client = reqwest::Client::new();
    let res = client.post(url).json(&map).send().await;
    let final_res = match res {
        Ok(r) => r,
        Err(_) => {
            return Err("Error: posting request");
        }
    };
    let result = final_res.status();
    Ok(result)
}

#[derive(Deserialize, Debug)]
struct Token {
    pub token: String,
}

async fn login_post_helper(username: &str, password: &str) -> Result<Response, &'static str> {
    let url: String = format!("http://localhost:8001/login/");
    let mut map = HashMap::new();
    map.insert("username", username);
    map.insert("password", password);

    let client = reqwest::Client::new();
    let res = client.post(url).json(&map).send().await;

    let final_res = match res {
        Ok(r) => r,
        Err(_) => {
            return Err("Error: posting request");
        }
    };
    println!("{:?}", &final_res);
    Ok(final_res)
}

#[derive(Deserialize, Debug)]
pub struct Message {
    pub message_id: u32,
    pub chat_id: u32,
    pub sent_from: u32,
    pub message: String,
}

pub struct CreateUserCommand {
    pub command: String,
    pub status: StatusCode,
}
impl CreateUserCommand {
    pub async fn build(args: Vec<&str>) -> Result<StatusCode, &'static str> {
        if args.len() < 4 {
            return Err("not enough arguments");
        }
        if args.len() > 4 {
            return Err("too many arguments");
        }
        let create: String = args[0].to_string();
        let username: String = args[1].to_string();
        let password: String = args[2].to_string();

        let res = user_post_helper(&username, &password).await?;

        Ok(res)
    }
}
#[derive(Deserialize, Debug)]
pub struct User {
    pub user_id: u32,
    pub username: String,
    pub password: String,
}
pub struct UserCommand {
    pub command: String,
    pub username: u32,
}

impl UserCommand {
    pub async fn build(args: Vec<&str>) -> Result<UserCommand, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        if args.len() > 2 {
            return Err("too many arguments");
        }
        let command: String = args[0].to_string();
        let username: u32 = args[1].to_string().parse::<u32>().unwrap();

        let user: UserCommand;
        user = UserCommand { command, username };
        let url = format!("http://localhost:8001/users/{}", user.username);

        let get_res = reqwest::get(url).await;

        let final_result = match get_res {
            Ok(r) => r.json::<User>().await,
            Err(_) => {
                return Err("Error: fetching request");
            }
        };

        let final_result_result = match final_result {
            Ok(r) => r,
            Err(_) => {
                return Err("Error: fetching request");
            }
        };
        println!("{:?}", final_result_result);
        Ok(user)
    }
}

impl fmt::Debug for UserCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.command, self.username)
    }
}

pub struct MessageCommand {
    pub command: String,
    pub user: String,
    pub message: String,
}

impl MessageCommand {
    pub fn build(args: Vec<&str>) -> Result<MessageCommand, &'static str> {
        if args.len() < 4 {
            return Err("not enough arguments");
        }
        if args.len() > 4 {
            return Err("too many arguments");
        }
        let command: String = args[0].to_string();
        let user: String = args[1].to_string();
        let message: String = args[3].to_string();

        let mes = MessageCommand { command, user, message };

        message_post_helper(&mes.user, &mes.message);
        return Ok(mes);
    }

    pub fn send() {}
}

impl fmt::Debug for MessageCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} '{}'", self.command, self.user, self.message)
    }
}

pub struct ChatCommand {
    pub command: String,
    pub user: String,
    pub depth: u32,
}

//TODO: fix this issue
impl ChatCommand {
    pub async fn build(args: Vec<&str>) -> Result<ChatCommand, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        if args.len() > 3 {
            return Err("too many arguments");
        }
        let command: String = args[0].to_string();
        let user: String = args[1].to_string();
        let depth: u32 = args[2].to_string().parse::<u32>().unwrap();

        let chat = ChatCommand { command, user, depth };
        let mes = message_get_helper(&chat.user).await;
        let result: Result<Message, &str> = match mes {
            Ok(r) => Ok(r),
            Err(_) => {
                return Err("Error: fetching request");
            }
        };
        println!("{:?}", result);
        return Ok(chat);
    }
}

impl fmt::Debug for ChatCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.command, self.user, self.depth)
    }
}

pub struct DeleteCommand {
    pub command: String,
    pub user: String,
}

impl DeleteCommand {
    pub fn build(args: Vec<&str>) -> Result<DeleteCommand, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        if args.len() > 2 {
            return Err("too many arguments");
        }
        let command: String = args[0].to_string();
        let user: String = args[1].to_string();

        Ok(DeleteCommand { command, user })
    }
}

impl fmt::Debug for DeleteCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.command, self.user)
    }
}

pub struct ListCommand {
    pub command: String,
    pub depth: u32,
}

impl ListCommand {
    pub fn build(args: Vec<&str>) -> Result<ListCommand, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        if args.len() > 2 {
            return Err("too many arguments");
        }
        let command: String = args[0].to_string();
        let depth: u32 = args[1].to_string().parse::<u32>().unwrap();

        Ok(ListCommand { command, depth })
    }
}

impl fmt::Debug for ListCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.command, self.depth)
    }
}

#[derive(Deserialize, Debug)]
pub struct LoginCommand {
    pub command_user: String,
}

impl LoginCommand {
    pub async fn build(args: Vec<&str>) -> Result<String, &'static str> {
        if args.len() < 4 {
            return Err("not enough arguments");
        }
        if args.len() > 4 {
            return Err("too many arguments");
        }
        let command_user: String = args[0].to_string();
        let username: String = args[1].to_string();
        let command_pass: String = args[2].to_string();
        let password: String = args[3].to_string();

        let val = login_post_helper(&username, &password).await?.text().await;
        let fin = match val {
            Ok(v) => v,
            Err(_) => {
                return Err("Login Failed");
            }
        };
        let mut tok: Vec<&str> = fin.split("{").collect();
        let tok_step: Vec<&str> = tok[1].split("\"").collect();
        //let final_tok: Vec<&str> = tok_step[].split(":").collect();
        let final_final: String = tok_step[3].to_string();
        Ok(final_final)
        //Ok(LoginCommand { command_user })
    }
}

// impl fmt::Debug for ListCommand {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{} {}", self.command, self.depth)
//     }
// }
