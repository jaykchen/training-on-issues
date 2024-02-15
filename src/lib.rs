pub mod utils;

use octocrab::{
    models::issues::Issue,
    params::{issues::Sort, Direction, State},
    Octocrab,
};
use serde::{Deserialize, Serialize};

use std::collections::HashSet;
use utils::*;

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct Payload {
    pub number: u64,
    pub title: String,
    pub url: String,
    pub labels: Option<Vec<String>>,
    pub creator: String,
    pub essence: Option<String>,
}

pub async fn why_labels(
    issue: &Issue,
    _contributors_set: HashSet<String>,
) -> anyhow::Result<Payload> {
    let issue_creator_name = &issue.user.login;
    let issue_title = issue.title.to_string();
    let issue_number = issue.number;
    let issue_body = match &issue.body {
        Some(body) => body.to_string(),
        None => "".to_string(),
    };
    let issue_url = issue.url.to_string();
    let _source_url = issue.html_url.to_string();

    let labels = issue
        .labels
        .iter()
        .map(|lab| lab.name.clone())
        .collect::<Vec<String>>();

    let issue_body = issue_body.chars().take(32_000).collect::<String>();

    // println!("issue_title: {}", issue_title);

    let system_prompt =
        String::from("You're a programming bot tasked to analyze GitHub issues data.");

    let user_prompt = format!(
        r#"You are tasked with refining and simplifying the information presented in a GitHub issue while keeping the original author's perspective. Think of it as if the original author decided to restate their issue in a more concise manner, focusing on clarity and brevity without losing the essence or the technical specifics of their original message.
        Issue text: {issue_body}
        Instructions:
        - Condense the issue's content by focusing on the primary technical details, proposals, and challenges mentioned, as if restating them directly in the author's voice.
        - Maintain the original tone and perspective of the author. Your summary should read as though the author themselves is offering a clearer, more straightforward version of their original text.
        - Include key actionable items, technical specifics, and any proposed solutions or requests made by the author, ensuring these elements are presented succinctly.
        - Avoid shifting to a third-person narrative. The goal is to simplify the author's message without altering the viewpoint from which it is delivered.
        - Preserve any direct quotes, technical terms, or specific examples the author used to illustrate their points, but ensure they are integrated into the summary seamlessly and without unnecessary elaboration.
        - Aim for a summary that allows quick grasping of core points and intentions, aiding efficient understanding and response. 
        - Explicitly remove unnecessary new lines, spaces, and combine multiple new lines into one. Pay special attention to avoid consecutive new lines (i.e., '\n\n') in your summary. Escape special characters as needed for command line compatibility.
        - Do not add extraneous wordings or notations like 'summary', '###', etc., from the original text.
        Your summary's effectiveness in capturing the essence while staying true to the author's intent is crucial for accurate content analysis and label assignment training."#
    );

    let essence = chat_inner(&system_prompt, &user_prompt).await?;

    Ok(Payload {
        number: issue_number,
        title: issue_title,
        url: issue_url,
        labels: Some(labels),
        creator: issue_creator_name.to_string(),
        essence: Some(essence),
    })
}

pub async fn get_issues() -> anyhow::Result<Vec<Issue>> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
    let octocrab = Octocrab::builder()
        .personal_token(token)
        .build()
        .expect("token invalid");

    let issue_handle = octocrab.issues("wasmedge", "wasmedge");
    let mut res = Vec::new();

    for n in 1..99_u8 {
        let list = issue_handle
            .list()
            .state(State::Open)
            // .milestone(1234)
            // .assignee("ferris")
            // .creator("octocrab")
            // .mentioned("octocat")
            // .labels(&labels)
            .sort(Sort::Created)
            .direction(Direction::Descending)
            .per_page(100)
            .page(n)
            .send()
            .await?;

        for iss in list.items {
            if iss.pull_request.is_some() {
                continue;
            }
            // println!("{:?}", iss.title);
            res.push(iss);
        }
    }

    Ok(res)
}
