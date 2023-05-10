use chat_gpt_rs::prelude::*;
use colored::*;
use std::io::{stdin, stdout, Write};

fn get_user_input() -> String {
    let mut s = String::new();
    print!("{}", "> ".red());
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("error: not correct input string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    //println!("you typed: {}", s);
    return s;
}

#[tokio::main]
async fn main() {
    let token = Token::new("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");
    let api = Api::new(token);

    println!("{}", "chat gpt v3.5 & v4 client v0.1.1".yellow());
    println!(" ");

    loop {
        let input_string = get_user_input();

        if input_string == "exit".to_string() || input_string == "quit".to_string() {
            break;
        }

        let request = Request {
            model: Model::Gpt35Turbo,
            messages: vec![Message {
                role: "user".to_string(),
                content: input_string,
            }],
            ..Default::default()
        };

        let response = api.chat(request).await;
        if let Ok(response) = response {
            let response_message = response.choices[0].message.content.clone();
            println!("{}", response_message.green());
        } else {
            println!("Error: {:?}", response.err());
        }
    }
}
