use std::fmt;
use reqwest;
use serde::Deserialize;

pub struct UserCommand {
    pub command: String,
    pub username: u32,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub user_id: u32,
    pub username: String,
    pub password: String,
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

        Ok(MessageCommand { command, user, message })
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
    pub user: u32,
    pub depth: u32,
}

impl ChatCommand {
    pub fn build(args: Vec<&str>) -> Result<ChatCommand, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        if args.len() > 3 {
            return Err("too many arguments");
        }
        let command: String = args[0].to_string();
        let user: u32 = args[1].to_string().parse::<u32>().unwrap();
        let depth: u32 = args[2].to_string().parse::<u32>().unwrap();

        Ok(ChatCommand { command, user, depth })
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
