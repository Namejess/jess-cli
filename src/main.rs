use hyper::body::Buf;
use hyper::{header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};
use std::io::{stdin, stdout, Write};

#[derive(Deserialize, Debug)]
struct OAIChoices {
    text: String,
    index: u8,
    logprobs: Option <u8>,
    finish_reason: String
}

#[derive(Deserialize, Debug)]
struct OAIResponse {
    id: Option<String>,
    object: Option<String>,
    created: Option<u64>,
    model: Option<String>,
    choices: Vec<OAIChoices>
}

#[derive(Serialize, Debug)]
struct OAIRequest {
    prompt: String,
    max_tokens: u32
}

#[tokio::main]
async fn main () -> Result < (), Box < dyn std::error::Error + Send + Sync >> {
    let https = HttpsConnector::new();
    let client = Client::builder().build(https);
    let uri = "https://api.openai.com/v1/engines/text-davinci-003/completions";

    let preamble = "This is an interactive example of OpenAI's API. You can ask it questions and it will try to answer them. It's not very good yet, but it will get better over time. Try it out! Type Ctrl-D or send an empty message to quit.";

    let oai_token: String = "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".to_string();
    let auth_header_val = format!("Bearer {}", oai_token);
    

    println!("{esc}c", esc = 27 as char);

    
    loop {
        print!("🦀 ==> ");
        stdout().flush().unwrap();
        let mut user_text = String::new();

        stdin().read_line(&mut user_text).expect("Failed to read line");

        println!("");

        let mut sp = Spinner::new(Spinners::Dots9, "Jess est en train de réfléchir...".to_string());

        let oai_request = OAIRequest {
            prompt: format!("{} {}", preamble, user_text),
            max_tokens: 100
        };

        let body = Body::from(serde_json::to_vec(&oai_request)?);
        // let body = Body::from(serde_json::to_vec(&oai_request)?);

        let req = Request::post(uri)
            .header(header::CONTENT_TYPE, "application/json")
            .header("Authorization", &auth_header_val)
            .body(body)
            .unwrap();

        let res = client.request(req).await?;

        let body = hyper::body::aggregate(res).await?;

        let json: OAIResponse = serde_json::from_reader(body.reader())?;

        sp.stop();

        print!("");

        println!("{}", json.choices[0].text);

    }

    

}

