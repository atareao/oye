mod ia;

use ia::IABot;
use inquire::{Text, Select};
use spinners::{Spinner, Spinners};
use std::str::FromStr;
use tracing_subscriber::{EnvFilter,
    layer::SubscriberExt, util::SubscriberInitExt};
use tracing::{info, error};
use process_stream::{Process, ProcessExt, StreamExt};

#[tokio::main]
async fn main() {
    let iabot = IABot::new().await;
    tracing_subscriber::registry()
        .with(EnvFilter::from_str(iabot.get_log_level()).unwrap())
        .with(tracing_subscriber::fmt::layer())
        .init();
    let question = Text::new("Di:").prompt();
    match question{
        Ok(question) => {
            info!("Question: {}", question);
            let mut spinner = Spinner::new(Spinners::Dots9, "🤔".to_string());
            match iabot.ask(&question).await{
                Ok (command) => {
                    spinner.stop();
                    info!("Command: {}", &command);
                    let ans = Select::new(
                        &format!("Ejecuto '{}'?", &command),
                        vec!["Si", "No"]).prompt();
                    match ans{
                        Ok(seleccion) => {
                            if seleccion == "Si" {
                                let mut process: Process = command
                                    .split(" ")
                                    .collect::<Vec<&str>>()
                                    .into();
                                let outputs = process.spawn_and_stream()
                                    .unwrap()
                                    .collect::<Vec<_>>()
                                    .await;
                                println!("Resultado: {outputs:#?}");
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
        },
        Err(e) => {
            println!("No se que ha pasado, repite, porfa");
            error!("Error: {}", e);
        }

    }
}
