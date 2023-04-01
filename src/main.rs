mod ia;

use ia::IABot;
use inquire::{Select, Text};
use spinners::{Spinner, Spinners};
use std::{str::FromStr, env};
use tracing_subscriber::{EnvFilter,
    layer::SubscriberExt, util::SubscriberInitExt};
use tracing::{debug, info, error};
use colored::Colorize;
use std::{process, path::PathBuf};

#[tokio::main]
async fn main() {
    let config = match get_config().await{
        Some(path) => path,
        None => {
            let mut path = std::env::current_dir().unwrap();
            path.push("oye.yml");
            IABot::write_default(&path).await;
            path
        }
    };
    let mut iabot = IABot::read_content(&config).await;
    while iabot.get_token() == ""{
        let message = format!("Error: Token in {} is empty", &config.to_str().unwrap());
        println!("{}", message.red());
        let api_key = Text::new("Set the API key (https://platform.openai.com/account/api-keys):")
            .prompt();
        match api_key{
            Ok(api_key) => {
                debug!("Response: '{}'", &api_key);
                if !api_key.is_empty() {
                    iabot.set_token(api_key);
                    iabot.save(&config).await;
                    break;
                }
            },
            Err(e) => {
                println!("An error happened: {}", e);
                process::exit(1);
            }
        }
    }
    tracing_subscriber::registry()
        .with(EnvFilter::from_str(iabot.get_log_level()).unwrap())
        .with(tracing_subscriber::fmt::layer())
        .init();
    let question = env::args().collect::<Vec<_>>()[1..].join(" ");
    info!("Question: {}", question);
    let mut spinner = Spinner::new(Spinners::Dots9, "🤔  ".to_string());
    match iabot.ask(&question).await{
        Ok (command) => {
            spinner.stop();
            info!("Command: {}", &command);
            let command = command.replace("\"", "");
            info!("Command: {}", &command);
            let ans = Select::new(
                &format!("¿Ejecuto {}?", &command.red()),
                vec!["Si".green(), "No".blue()]).prompt();
            match ans{
                Ok(seleccion) => {
                    if seleccion == "Si".green() {
                        let mut spinner = Spinner::new(Spinners::Dots9, String::new());
                        let args = command.split(" ").collect::<Vec<_>>();
                        debug!("{:?}", args);
                        let mut command = tokio::process::Command::new(&args[0]);
                        if args.len() > 1{
                            command.args(&args[1..]);
                        }
                        debug!("Command: {:?}", command);
                        let response = command
                            .env("PWD", std::env::current_exe().unwrap())
                            .output()
                            .await
                            .unwrap();
                        // let mut child = command.spawn().unwrap();
                        debug!("Response: {:?}", response);
                        // child.wait().await.unwrap();
                        // let output = child.stdout.take().unwrap();
                        spinner.stop_with_newline();
                        let output = if response.status.success(){
                            std::str::from_utf8(&response.stdout).unwrap().blue()
                        }else{
                            std::str::from_utf8(&response.stderr).unwrap().red()
                        };
                        if output.is_empty(){
                            println!("{}", "No hay nada!".yellow());
                        }else{
                            println!("{}", output);
                        }
                    }else{
                        println!("{}", "Otra vez será".blue());
                    }
                },
                Err(e) => {
                    println!("{}", "Algo ha pasado, elige de nuevo".red());
                    error!("{}", e);
                }
            };
        },
        Err(e) => {
            spinner.stop();
            error!("Error: {}", e);
            println!("Error: {}", e.to_string().red());
        }
    }
}

async fn get_config() -> Option<PathBuf>{
    let mut current_path = std::env::current_dir().unwrap();
    current_path.push("oye.yml");
    debug!("Current path: {}", current_path.display());
    if(tokio::fs::metadata(&current_path)).await.is_ok(){
        return Some(current_path);
    }
    let mut exe_path = std::env::current_exe().unwrap();
    exe_path.push("oye.yml");
    debug!("Exe path: {}", exe_path.display());
    if(tokio::fs::metadata(&exe_path)).await.is_ok(){
        return Some(exe_path);
    }
    let mut home_path = dirs::home_dir().unwrap();
    debug!("Home path: {}", home_path.display());
    home_path.push(".oye.yml");
    if(tokio::fs::metadata(&home_path)).await.is_ok(){
        return Some(home_path);
    }
    let mut config_path = dirs::config_dir().unwrap();
    config_path.push("oye.yml");
    debug!("Config path: {}", config_path.display());
    if(tokio::fs::metadata(&config_path)).await.is_ok(){
        return Some(config_path);
    }
    let mut config_folder = dirs::config_dir().unwrap();
    config_folder.push("oye");
    config_folder.push("config.yml");
    debug!("Config folder: {}", config_folder.display());
    if(tokio::fs::metadata(&config_folder)).await.is_ok(){
        return Some(config_folder);
    }
    None
}
