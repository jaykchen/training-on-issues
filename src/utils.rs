use async_openai::{
    types::{
        // ChatCompletionFunctionsArgs, ChatCompletionRequestMessage,
        ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs,
        // ChatCompletionTool, ChatCompletionToolArgs, ChatCompletionToolType,
        CreateChatCompletionRequestArgs,
        // FinishReason,
    },
    Client,
};

pub fn squeeze_fit_remove_quoted(inp_str: &str, max_len: u16, split: f32) -> String {
    let mut body = String::new();
    let mut inside_quote = false;

    for line in inp_str.lines() {
        if line.contains("```") || line.contains("\"\"\"") {
            inside_quote = !inside_quote;
            continue;
        }

        if !inside_quote {
            let cleaned_line = line
                .split_whitespace()
                .filter(|word| word.len() < 150)
                .collect::<Vec<&str>>()
                .join(" ");
            body.push_str(&cleaned_line);
            body.push('\n');
        }
    }

    let body_words: Vec<&str> = body.split_whitespace().collect();
    let body_len = body_words.len();
    let n_take_from_beginning = ((body_len as f32) * split) as usize;
    let n_keep_till_end = body_len - n_take_from_beginning;

    // Range check for drain operation
    let drain_start = if n_take_from_beginning < body_len {
        n_take_from_beginning
    } else {
        body_len
    };

    let drain_end = if n_keep_till_end <= body_len {
        body_len - n_keep_till_end
    } else {
        0
    };

    let final_text = if body_len > (max_len as usize) {
        let mut body_text_vec = body_words.to_vec();
        body_text_vec.drain(drain_start..drain_end);
        body_text_vec.join(" ")
    } else {
        body
    };

    final_text
}

/* pub async fn chain_of_chat(
    sys_prompt_1: &str,
    usr_prompt_1: &str,
    chat_id: &str,
    gen_len_1: u16,
    usr_prompt_2: &str,
    gen_len_2: u16,
    error_tag: &str
) -> anyhow::Result<String> {
    use reqwest::header::{ HeaderValue, CONTENT_TYPE, USER_AGENT };
    let token = env::var("DEEP_API_KEY").expect("DEEP_API_KEY must be set");

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(USER_AGENT, HeaderValue::from_static("MyClient/1.0.0"));
    let config = LocalServiceProviderConfig {
        // api_base: String::from("http://52.37.228.1:8080/v1"),
        api_base: String::from("http://52.37.228.1:8080/v1"),
        headers: headers,
        api_key: Secret::new(token),
        query: HashMap::new(),
    };

    let model = "mistralai/Mistral-7B-Instruct-v0.1";
    let client = OpenAIClient::with_config(config);

    let mut messages = vec![
        ChatCompletionRequestSystemMessageArgs::default()
            .content(sys_prompt_1)
            .build()
            .expect("Failed to build system message")
            .into(),
        ChatCompletionRequestUserMessageArgs::default().content(usr_prompt_1).build()?.into()
    ];
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(gen_len_1)
        .model(model)
        .messages(messages.clone())
        .build()?;

    let chat = client.chat().create(request).await?;

    match chat.choices[0].message.clone().content {
        Some(res) => {
            log::info!("step 1 Points: {:?}", res);
        }
        None => {
            return Err(anyhow::anyhow!(error_tag.to_string()));
        }
    }

    messages.push(
        ChatCompletionRequestUserMessageArgs::default().content(usr_prompt_2).build()?.into()
    );

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(gen_len_2)
        .model(model)
        .messages(messages)
        .build()?;

    let chat = client.chat().create(request).await?;

    match chat.choices[0].message.clone().content {
        Some(res) => {
            log::info!("step 2 Raw: {:?}", res);
            Ok(res)
        }
        None => {
            return Err(anyhow::anyhow!(error_tag.to_string()));
        }
    }
} */
pub async fn chat_inner(system_prompt: &str, user_input: &str) -> anyhow::Result<String> {
    let client = Client::new();
    let messages = vec![
        ChatCompletionRequestSystemMessageArgs::default()
            .content(system_prompt)
            .build()
            .expect("Failed to build system message")
            .into(),
        ChatCompletionRequestUserMessageArgs::default()
            .content(user_input)
            .build()?
            .into(),
    ];
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model("gpt-3.5-turbo-1106")
        .messages(messages.clone())
        .build()?;

    let chat = client.chat().create(request).await?;

    // let check = chat.choices.get(0).clone().unwrap();
    // send_message_to_channel("ik8", "general", format!("{:?}", check)).await;

    match chat.choices[0].message.clone().content {
        Some(res) => {
            println!("Chat response: {}", res);

            Ok(res)
        }
        None => Err(anyhow::anyhow!("Failed to get response from chat")),
    }
}

/* pub fn parse_summary_from_raw_json(input: &str) -> anyhow::Result<String> {
    use regex::Regex;
    let parsed = match serde_json::from_str(input) {
        Ok(v) => v,
        Err(e) => {
            log::error!("Error parsing JSON: {:?}", e);
            // Attempt to extract fields using regex if JSON parsing fails
            let mut values_map = std::collections::HashMap::new();
            let keys = ["impactful", "alignment", "patterns", "synergy", "significance"];
            for key in keys.iter() {
                let regex_pattern = format!(r#""{}":\s*"([^"]*)""#, key);
                let regex = Regex::new(&regex_pattern)
                    .map_err(|_| anyhow::Error::msg("Failed to compile regex pattern"))
                    .expect("Failed to compile regex pattern");
                if let Some(captures) = regex.captures(input) {
                    if let Some(value) = captures.get(1) {
                        values_map.insert(*key, value.as_str().to_string());
                    }
                }
            }

            if values_map.len() != keys.len() {
                return Err(anyhow::Error::msg("Failed to extract all fields from JSON"));
            }

            Value::Object(
                values_map
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), Value::String(v)))
                    .collect()
            )
        }
    };

    let mut output = String::new();
    let keys = ["impactful", "alignment", "patterns", "synergy", "significance"];

    for key in keys.iter() {
        if let Some(value) = parsed.get(*key) {
            if value.is_string() {
                if !output.is_empty() {
                    output.push_str(" ");
                }
                output.push_str(value.as_str().unwrap());
            }
        }
    }

    Ok(output)
} */
