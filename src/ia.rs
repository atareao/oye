use serde::{Serialize, Deserialize};
use tokio::fs::read_to_string;
use std::{process, path::PathBuf};
use serde_json::json;
use dirs::config_dir;
use tracing::{info, debug, error};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IABot{
    #[serde(default = "get_default_log_level")]
    log_level: String,
    #[serde(default = "get_default_base_url")]
    base_url: String,
    #[serde(default = "get_default_endpoint")]
    endpoint: String,
    #[serde(default = "get_default_models_endpoint")]
    models_endpoint: String,
    #[serde(default = "get_default_token")]
    token: String,
    #[serde(default = "get_default_model")]
    model: String,
    // What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.
    #[serde(default = "get_default_temperature")]
    temperature: String,
    #[serde(default = "get_default_content")]
    content: String,
}

fn get_default_log_level() -> String{
    return "info".to_string();
}

fn get_default_base_url() -> String {
    return "https://api.openai.com".to_string();
}

fn get_default_endpoint() -> String {
    return "v1/chat/completions".to_string();
}

fn get_default_token() -> String {
    return "".to_string();
}

fn get_default_models_endpoint() -> String {
    return "v1/models".to_string();
}

fn get_default_model() -> String {
    return "gpt-3.5-turbo".to_string();
}

fn get_default_temperature() -> String {
    return "1".to_string();
}

fn get_default_content() -> String {
    return "Eres un asistente útil. Generarás comandos '$SHELL' en función de la entrada del usuario. Tu respuesta debe contener SOLO el comando y NO una explicación. NUNCA uses saltos de línea para separar comandos, en su lugar usa ; o &&. El directorio de trabajo actual es '$PWD'.".to_string();
}

impl IABot{

    pub async fn new() -> Self{
        let mut config_file = PathBuf::new();
        match config_dir(){
            Some(dir) => {
                config_file.push(dir);
                config_file.push("eh");
                if(tokio::fs::metadata(&config_file)).await.is_ok() == false{
                    let _ = tokio::fs::create_dir_all(&config_file).await;
                }
                config_file.push("config.yml");
                if tokio::fs::metadata(&config_file).await.is_ok() == false{
                    Self::write(&config_file).await
                }
                return Self::read_content(&config_file).await;
            }
            None =>{
                println!("Not exists config dir");
                process::exit(1);
            }

        }
    }

    pub fn get_log_level(&self) -> &str {
        &self.log_level
    }

    pub async fn ask(&self, question: &str) -> Result<String, reqwest::Error>{
        let url = format!("{}/{}", self.base_url, self.endpoint);
        debug!("Url: {}", url);
        debug!("Question: {}", question);
        let client = reqwest::Client::new();
        let response = client.post(&url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&json!({
                "model": self.model,
                "messages": [
                    {"role": "system", "content": &self.content},
                    {"role": "user", "content": format!("'{}'", question)}
                ]}
            ))
            .send()
            .await
            .unwrap()
            .json::<serde_json::Value>()
            .await
            .unwrap();
        debug!("Response: {:#?}", response);
        let command = response.get("choices")
            .unwrap()
            .as_array()
            .unwrap()[0]
            .get("message")
            .unwrap()
            .get("content")
            .unwrap()
            .as_str()
            .unwrap();
        // "choices": Array [
        //     Object {
        //         "finish_reason": String("stop"),
        //         "index": Number(0),
        //         "message": Object {
        //             "content": String("ls"),
        //             "role": String("assistant"),
        //         },
        //     },
        // ],
        // "created": Number(1678996561),
        // "id": String("chatcmpl-6unzdarhMpabCQ6gegPPcriXIUSpn"),
        // "model": String("gpt-3.5-turbo-0301"),
        // "object": String("chat.completion"),
        // "usage": Object {
        //     "completion_tokens": Number(2),
        //     "prompt_tokens": Number(96),
        //     "total_tokens": Number(98),
        // },
        debug!("Command: {}", command);
        Ok(command.to_string())
    }
    fn default() -> Self{
        Self{
            log_level: get_default_log_level(),
            base_url: get_default_base_url(),
            endpoint: get_default_endpoint(),
            models_endpoint: get_default_models_endpoint(),
            token: get_default_token(),
            model: get_default_model(),
            temperature: get_default_temperature(),
            content: get_default_content(),
        }
    }
    async fn write(file: &PathBuf){
        let default = Self::default();
        let _ = tokio::fs::write(file,serde_yaml::to_string(&default).unwrap().as_bytes()).await;
    }
    async fn read_content(file: &PathBuf) -> IABot{
        info!("File to read: {:?}", file);
        let content = match read_to_string(file)
            .await {
                Ok(value) => {
                    debug!("Content read: {}", value);
                    value
                },
                Err(e) => {
                    error!("Error: {}", e);
                    println!("Error with config file `config.yml`: {}",
                        e.to_string());
                    process::exit(1);
                }
            };
        match serde_yaml::from_str(&content){
            Ok(configuration) => configuration,
            Err(e) => {
                error!("Error: {}", e);
                println!("Error with config file `config.yml`: {}",
                    e.to_string());
                process::exit(1);
            }
        }
    }

}

