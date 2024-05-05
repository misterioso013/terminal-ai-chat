# terminal-ai-chat

Tenha acesso ao Gemini no seu terminal

## Instalação

### Windows

> Caso esteja com problemas para executar os comandos abaixo, por favor, vá na barra de pesquisa, digite `prompt`e execute como adminstrador.

Faça o download do arquivo `.exe` da última versão [aqui](https://github.com/misterioso013/terminal-ai-chat/releases).

Mova o arquivo `ai.exe` para o caminho `C:\Users\SEU_NOME` e agora você estará pronto para criar as variáveis de ambiente.

Abra seu terminal como administrador e cole o comando abaixo:

```bash
setx /M PATH "%PATH%;C:\Users\SEU_NOME"
```

> Se tudo correr bem você poderá fechar o terminal e ao digitar `ai oi` você receberá algo como "`Chave de API Google AI não encontrada!...`"

Agora vamos configurar a chave de API do Google AI que você poderá encontrar [aqui](https://aistudio.google.com/app/apikey)

Após criar a chave de API, abra o terminal como administrador e rode o comando abaixo alterando `your_api_key` pela sua chave de API que foi gerada no passo anterior.

```bash
setx /M GOOGLE_AI_API_KEY "your_api_key"
```

> Se tudo ocorrer bem, você estará pronto para usar `ai Oi`.

![example](https://i.ibb.co/wMTT9Py/Captura-de-tela-2024-05-05-205113.png)
