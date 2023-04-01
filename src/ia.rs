use hyper::Request;
use serde::{Serialize, Deserialize};
use tokio::fs::read_to_string;
use std::{process, path::PathBuf};
use serde_json::json;
use tracing::{info, debug, error};
use std::error::Error;
use std::fmt;

use hyper::Client;

#[derive(Debug)]
pub struct IABotError{
    message: String,
}

impl IABotError{
    pub fn new(message: &str) -> Self{
        Self {
            message: message.into()
        }
    }
}


// Display implementation is required for std::error::Error.
impl fmt::Display for IABotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //write!(f, "Address is localhost")
        write!(f, "{}", self.message)
    }
}

impl Error for IABotError {} // Defaul


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
    return "api.openai.com".to_string();
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
    return "Eres un asistente útil. Generarás comandos '$SHELL' en función de la entrada del usuario. Tu respuesta debe contener SOLO el comando y NO una explicación. NUNCA uses saltos de línea para separar comandos, en su lugar usa ; o &&. El directorio de trabajo actual es '$PWD'".to_string();
}


impl IABot{

    pub fn get_log_level(&self) -> &str {
        &self.log_level
    }

    pub fn get_token(&self) -> &str{
        &self.token
    }

    pub fn set_token(&mut self, token: String){
        self.token = token;
    }

    pub async fn save(&self, path: &PathBuf){
        let _ = tokio::fs::write(
            path,
            serde_yaml::to_string(&self).unwrap().as_bytes()
        ).await;
    }

    pub async fn ask(&self, question: &str) -> Result<String, Box<dyn Error + 'static>>{
        let url = format!("https://{}/{}", self.base_url, self.endpoint);
        debug!("Url: {}", url);
        debug!("Question: {}", question);
        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_only()
            .enable_http1()
            .build();
        let client = Client::builder().build(https);
        let body = serde_json::to_string(&json!({
            "model": self.model,
            "messages": [
                {"role": "system", "content": &self.content},
                {"role": "user", "content": format!("'{}'", question)}
            ]
        })).unwrap();
        let request = Request::builder()
            .method("POST")
            .uri(url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.token))
            .body(hyper::Body::from(body))
            .unwrap();
        match client.request(request).await{
            Ok(resp) => {
                let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;
                // Convert the body bytes to utf-8
                let body = String::from_utf8(body_bytes.to_vec()).unwrap();
                debug!("{}", &body);
                let data: serde_json::Value = serde_json::from_str(&body).unwrap();
                if data.get("error").is_some(){
                    let error = data.get("error").unwrap();
                    let message = error.get("message").unwrap().as_str().unwrap();
                    Err(Box::new(IABotError::new(message)))
                }else{
                    let command = data.get("choices")
                        .unwrap()
                        .as_array()
                        .unwrap()[0]
                        .get("message")
                        .unwrap()
                        .get("content")
                        .unwrap()
                        .as_str()
                        .unwrap();
                    Ok(command.to_string())
                }
            },
            Err(e) => {
                error!("{}", e);
                Err(Box::new(e))
            }
        }
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

    pub async fn write_default(file: &PathBuf){
        let default = Self::default();
        let _ = tokio::fs::write(
            file,
            serde_yaml::to_string(&default).unwrap().as_bytes()).await;
    }

    pub async fn read_content(file: &PathBuf) -> IABot{
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

