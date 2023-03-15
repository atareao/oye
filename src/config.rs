use std::{process, path::PathBuf};

use dirs;
use tokio;

const DEFAULT_BASE_URL: &'static str = "https://api.openai.com";
const DEFAULT_ENDPOINT: &'static str = "v1/chat/completions";
const DEFAULT_TOKEN: &'static str = "";
const DEFAULT_MODEL: &'static str = "gpt-3.5.turbo";
const DEFAULT_CONTENT: &'static str = "Eres un asistente útil. Generarás comandos '$SHELL' en función de la entrada del usuario. Tu respuesta debe contener SOLO el comando y NO una explicación. NUNCA uses saltos de línea para separar comandos, en su lugar usa ; o &&. El directorio de trabajo actual es '$cwd'.";

pub struct Config{
    base_url: String,
    endpoint: String,
    token: String,
    model: String,
    content: String,
}

impl Config{
    pub fn read() -> Self{

    }

    async fn create() -> Self{
        let home_dir = match dirs::home_dir() {
            Some(home_dir) => home_dir,
            None => {
                println!("Can't continue. Not exists HOME dir");
                process::exit(1);
            }
        };
        let main_config_dir = match dirs::config_dir(){
            Some(main_config_dir) => main_config_dir,
            None => {
                let mut main_config_dir = home_dir.clone();
                main_config_dir.push(".config");
                tokio::fs::create_dir(main_config_dir).await;
                main_config_dir
            }
        };
        tokio::fs::create_dir_all
        let config_dir = match dirs::config_dir(){
            Some(config_dir) => config_dir,
            None => {
                let mut config_dir = home_dir.clone();
                config_dir.push("di");
                tokio::fs::create_dir(config_dir).await;
                config_dir
            }
        };
        let config_file =

    }
}
