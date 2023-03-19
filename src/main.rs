mod ia;

use ia::IABot;
use inquire::Select;
use spinners::{Spinner, Spinners};
use std::{str::FromStr, env};
use tracing_subscriber::{EnvFilter,
    layer::SubscriberExt, util::SubscriberInitExt};
use tracing::{debug, info, error};
use colored::Colorize;

#[tokio::main]
async fn main() {
    let iabot = IABot::new().await;
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
                        let response = command.output().await.unwrap();
                        // let mut child = command.spawn().unwrap();
                        debug!("Response: {:?}", response);
                        // child.wait().await.unwrap();
                        // let output = child.stdout.take().unwrap();
                        spinner.stop_with_newline();
                        println!("{}", std::str::from_utf8(&response.stdout)
                            .unwrap());
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
        }
    }
}
