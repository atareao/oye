# oye

Default configuration `oye.yml`,

```bash

log_level: none
base_url: api.openai.com
endpoint: v1/chat/completions
models_endpoint: v1/models
token:
model: gpt-3.5-turbo
temperature: '1'
content: Eres un asistente útil. Generarás comandos '$SHELL' en función de la entrada del usuario. Tu respuesta debe contener SOLO el comando y NO una explicación. NUNCA uses saltos de línea para separar comandos, en su lugar usa ; o &&. El directorio de trabajo actual es '.'.
```

