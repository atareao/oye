use std::{process, path::PathBuf};

use dirs;
use dotenv;
use tokio;

const DEFAULT_BASE_URL: &'static str = "https://api.openai.com";
const DEFAULT_ENDPOINT: &'static str = "v1/chat/completions";
const DEFAULT_MODELS_ENDPOINT: &'static str = "v1/models";
const DEFAULT_TOKEN: &'static str = "";
const DEFAULT_MODEL: &'static str = "gpt-3.5.turbo";
const DEFAULT_TEMPERATURE: &'static str = "1";
const DEFAULT_CONTENT: &'static str = "Eres un asistente útil. Generarás comandos '$SHELL' en función de la entrada del usuario. Tu respuesta debe contener SOLO el comando y NO una explicación. NUNCA uses saltos de línea para separar comandos, en su lugar usa ; o &&. El directorio de trabajo actual es '$cwd'.";

pub struct Config{
    base_url: String,
    endpoint: String,
    token: String,
    model: String,
    // What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.
    temperature: String,
    content: String,
}

impl Config{
    pub fn default() -> Self{
        Self{
            base_url: DEFAULT_BASE_URL.to_string(),
            endpoint: DEFAULT_BASE_URL.to_string(),
            token: DEFAULT_BASE_URL.to_string(),
            model: DEFAULT_BASE_URL.to_string(),
            temperature: DEFAULT_BASE_URL.to_string(),
            content: DEFAULT_BASE_URL.to_string(),
        }
    }
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
        let mut config = main_config_dir.clone();
        config.push("di");
        tokio::fs::create_dir_all(config).await;
        config.push("config.ini");
        if tokio::fs::try_exists(config).await.ok() == Some(true){
            Self::default()
        }else{
            dotenv::from_filename(config).ok();
            let base_url = match std::env::var("BASE_URL"){
                Ok(value) => value,
                Err(_) => DEFAULT_BASE_URL.to_string(),
            };
            let endpoint = match std::env::var("BASE_URL"){
                Ok(value) => value,
                Err(_) => DEFAULT_BASE_URL.to_string(),
            };
            let token = match std::env::var("BASE_URL"){
                Ok(value) => value,
                Err(_) => DEFAULT_BASE_URL.to_string(),
            };
            let model = match std::env::var("BASE_URL"){
                Ok(value) => value,
                Err(_) => DEFAULT_BASE_URL.to_string(),
            };
            let temperature = match std::env::var("BASE_URL"){
                Ok(value) => value,
                Err(_) => DEFAULT_BASE_URL.to_string(),
            };
            let content = match std::env::var("BASE_URL"){
                Ok(value) => value,
                Err(_) => DEFAULT_BASE_URL.to_string(),
            };
            Self{
                base_url,
                endpoint,
                token,
                model,
                temperature,
                content,
            }

        }
    }
}
