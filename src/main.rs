mod ia;

use ia::IABot;
use inquire::Select;
use spinners::{Spinner, Spinners};
use std::{str::FromStr, env};
use tracing_subscriber::{EnvFilter,
    layer::SubscriberExt, util::SubscriberInitExt};
use tracing::{debug, info, error};

#[tokio::main]
async fn main() {
    let iabot = IABot::new().await;
    tracing_subscriber::registry()
        .with(EnvFilter::from_str(iabot.get_log_level()).unwrap())
        .with(tracing_subscriber::fmt::layer())
        .init();
    let question = env::args().collect::<Vec<_>>()[1..].join(" ");
    info!("Question: {}", question);
    let mut spinner = Spinner::new(Spinners::Dots9, "🤔".to_string());
    match iabot.ask(&question).await{
        Ok (command) => {
            info!("Command: {}", &command);
            spinner.stop();
            let ans = Select::new(
                &format!("Ejecuto '{}'?", &command),
                vec!["Si", "No"]).prompt();
            match ans{
                Ok(seleccion) => {
                    if seleccion == "Si" {
                        let args = command.split(" ").collect::<Vec<_>>();
                        debug!("{:?}", args);
                        let mut command = tokio::process::Command::new(&args[0]);
                        if args.len() > 1{
                            command.args(&args[1..]);
                        }
                        debug!("Command: {:?}", command);
                        let mut child = command.spawn().unwrap();
                        child.wait().await.unwrap();
                        let output = child.stdout.take().unwrap();
                        println!("{:?}", output);
                    }else{
                        println!("Otra vez será");
                    }
                },
                Err(e) => {
                    println!("Algo ha pasado, elige de nuevo");
                    error!("{}", e)
                }
            };
        },
        Err(e) => {
            error!("Error: {}", e);
            spinner.stop_with_message(format!("No se: {}", e));
        }
    }
    spinner.stop_with_message("Esta es la respuesta: ".to_string());
}
