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

Si lo quieres utilizar desde un contenedor Docker, puedes utilizar el siguiente `Dockerfile`

```bash
FROM ubuntu:latest
RUN apt-get update && apt-get install -y curl gcc
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN /root/.cargo/bin/cargo install oye
COPY oye.yml /root/.oye.yml

ENTRYPOINT ["/root/.cargo/bin/oye"]
```

```bash
docker build -t atareao/oye . 
```

Y para utilizarlo,

```bash
docker run -it --rm atareao/oye encuentra todos los archivos con extensión .yml
```

Siempre puedes hacer un alias, para simplificar,
