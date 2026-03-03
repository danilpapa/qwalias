use std::{env};
use dotenvy::dotenv;
use crate::services::install_aliases::{install_aliases, Alias};
use crate::services::shell::reload_shell;

mod services;

fn main() {
    dotenv().ok();

    let args: Vec<String> = env::args()
        .collect();

    if args.len() == 3 {
        let alias = Alias {
            title: args[1]
                .clone(),
            execution: args[2]
                .clone(),
        };
        let result = install_aliases(alias);
        if let Err(e) = result {
            println!("{}", e);
        }
        println!("Successfully installed!");
        reload_shell();
    }
}