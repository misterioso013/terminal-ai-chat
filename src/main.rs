use colored::Colorize;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
    struct Request {
        contents: Vec<Content>,
        generation_config: GenerationConfig,
        safety_settings: Vec<SafetySetting>,
    }

    #[derive(Serialize, Deserialize)]
    struct Content {
        role: String,
        parts: Vec<Part>,
    }

    #[derive(Serialize, Deserialize)]
    struct Part {
        text: String,
    }

    #[derive(Serialize, Deserialize)]
    struct GenerationConfig {
        temperature: f32,
        topK: i32,
        topP: f32,
        max_output_tokens: i32,
        stop_sequences: Vec<String>,
    }

    #[derive(Serialize, Deserialize)]
    struct SafetySetting {
        category: String,
        threshold: String,
    }

#[tokio::main]
async fn main() {
    match std::env::var("GOOGLE_AI_API_KEY") {
        Ok(_) => {},
        Err(_) => {
            println!("{}{}{}{}", "Chave de API Google AI não encontrada! ".red(), "Por favor, defina a variável de ambiente ".red().bold(), "GOOGLE_AI_API_KEY ".red().bold().underline(), "com a sua chave de API Google AI".red());
            println!("{}{}","Consulte a documentação em ".white(), "https://github.com/misterioso013/terminal-ai-chat".white().bold().underline());
            return;
        }
    }

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("{}","You not passed any text".red());
        return;
    }
    let text = &args[1..].join(" ");

    let request = Request {
        contents: vec![
            Content {
                role: "user".to_string(),
                parts: vec![
                    Part {
                        text: text.to_string(),
                    }
                ]
            }
        ],
        generation_config: GenerationConfig {
            temperature: 1.0,
            topK: 0,
            topP: 0.95,
            max_output_tokens: 8192,
            stop_sequences: vec![],
        },
        safety_settings: vec![
            SafetySetting {
                category: "HARM_CATEGORY_HARASSMENT".to_string(),
                threshold: "BLOCK_MEDIUM_AND_ABOVE".to_string(),
            },
            SafetySetting {
                category: "HARM_CATEGORY_HATE_SPEECH".to_string(),
                threshold: "BLOCK_MEDIUM_AND_ABOVE".to_string(),
            },
            SafetySetting {
                category: "HARM_CATEGORY_SEXUALLY_EXPLICIT".to_string(),
                threshold: "BLOCK_MEDIUM_AND_ABOVE".to_string(),
            },
            SafetySetting {
                category: "HARM_CATEGORY_DANGEROUS_CONTENT".to_string(),
                threshold: "BLOCK_MEDIUM_AND_ABOVE".to_string(),
            },
        ],
    };

    let client = reqwest::Client::new();
    let response = client.post(format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-pro-latest:generateContent?key={}", std::env::var("GOOGLE_AI_API_KEY").unwrap()))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&request).unwrap())
        .send()
        .await
        .unwrap();
    let response_json: serde_json::Value = response.text().await.unwrap().parse().unwrap();
    let response_text = match response_json["candidates"][0]["content"]["parts"][0]["text"].as_str() {
        Some(text) => text,
        None => "Error to get response text",
    };
    println!("{}", termimad::term_text(response_text));
}
