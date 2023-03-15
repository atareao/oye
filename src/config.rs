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

    fn create() -> Self{

    }
}
