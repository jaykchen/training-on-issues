use chrono::Utc;
use dotenv::dotenv;
use octocrab::{ models::issues::Issue, params::{ issues::Sort, Direction, State }, Octocrab };
use serde_json::{ json, to_string, to_string_pretty };
use sqlx::{ postgres::{ PgPool, PgPoolOptions, Postgres }, FromRow, Row };
use std::{ collections::HashSet, fs::{ File, OpenOptions }, io::{ BufWriter, Write } };
use training_on_issues::{ get_issues, why_labels, Payload };

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let DATABASE_URL = String::from("postgres://postgres:password@127.0.0.1/json");
    let pool = PgPool::connect(&DATABASE_URL).await?;
    dotenv().ok();
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
    let octocrab = Octocrab::builder().personal_token(token).build().expect("token invalid");

    let contributors_set = HashSet::<String>::new();

    let file = File::open("issue_1707700814.json")?;

    let parsed: Vec<Payload> = serde_json::from_reader(file)?;

    let header = String::from("text");
    let mut file = File::create(format!("issue_train{}.jsonl", Utc::now().timestamp()))?;
    let mut file = BufWriter::new(file);
    let mut all_labels = HashSet::<String>::new();
    for iss in parsed {
        let title = iss.clone().title;
        // let labels = iss.clone().labels.unwrap_or_default().join(", ");
        let url = iss.clone().url;
        if 
        // labels.is_empty() | 
        url.contains("pull") {
            continue;
        }

        if let Some(labels) = iss.labels {
            for label in &labels {
                all_labels.insert(label.clone());
            }
        }
        // let creator = iss.clone().creator;
        // let essence = iss.essence.unwrap_or_default().replace("\n\n", "\n").replace("\n", " ");

        // let question = format!(
        //     "Can you assign labels to the GitHub issue titled `{title}` created by `{creator}`, stating `{essence}`?"
        // );
        // let answer = format!("The labels for this issue are `{labels}`.");

        // // Construct the JSON object for each issue
        // let completion =
        //     json!({
        //     "prompt": question,
        //     "completion": answer
        // });

        // Convert the JSON object to a string and write it to the file with a newline
        // writeln!(file, "{}", completion.to_string())?;
    }
    println!("{:?}", all_labels);
    return Ok(());
    let filename = format!("issue_{}.json", Utc::now().timestamp());

    let mut file = OpenOptions::new()
        .create(true) // Will create the file if it does not exist
        .write(true) // Open the file for writing
        .append(true) // Set the file to append mode
        .open(&filename)
        .expect("Failed to open file");

    let issue_handle = octocrab.issues("wasmedge", "wasmedge");

    for n in 1..99_u8 {
        let list = issue_handle
            .list()
            .state(State::Closed)
            // .milestone(1234)
            // .assignee("ferris")
            // .creator("octocrab")
            // .mentioned("octocat")
            // .labels(&labels)
            .sort(Sort::Created)
            .direction(Direction::Descending)
            .per_page(50)
            .page(n)
            .send().await?;

        // let mut out = vec![];

        for iss in list.clone().items {
            if iss.pull_request.is_some() {
                continue;
            }
            let mut payload = Payload {
                number: iss.number,
                title: iss.clone().title,
                url: iss.html_url.to_string(),
                labels: Some(
                    iss.labels
                        .iter()
                        .map(|l| l.name.to_string())
                        .collect::<Vec<String>>()
                ),
                creator: iss.clone().user.login,
                essence: iss.body.clone(),
            };

            let body_text = serde_json::to_string(&payload).expect("Failed to serialize data");
            file.write_all(body_text.as_bytes())?; // Write the serialized payload
            file.write_all(b",")?;
            // out.push(payload.clone());
        }

        if list.items.len() < 15 {
            break;
        }

        // let body_text = serde_json::to_string(&out).expect("Failed to serialize data");

        // file.write_all(body_text.as_bytes())?;

        // if n > 2 {
        //     break;
        // }
    }

    Ok(())
}

async fn add_payload(pool: &PgPool, payload: &Payload) -> anyhow::Result<i64> {
    let rec = sqlx
        ::query(
            "INSERT INTO payloads (number, title,  url, labels, creator, essence)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING id"
        )
        .bind(payload.number as i64) // Assuming the DB expects an integer
        .bind(&payload.title)
        .bind(&payload.url)
        .bind(&payload.labels) // Make sure labels are serialized if it's an array
        .bind(&payload.creator)
        .bind(&payload.essence)
        .fetch_one(pool).await?;

    Ok(rec.get("id"))
}
