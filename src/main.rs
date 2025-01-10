use log::info;
use serde::{Deserialize, Serialize};
use std::fs::write;

// * Implement the Serialize and Deserialize traits for the Paragraph struct
#[derive(Serialize, Deserialize)]
struct Chapter {
    name: String,
}

// * Implement the Serialize and Deserialize traits for the Article struct
#[derive(Serialize, Deserialize)]
struct Article {
    title: String,
    author: String,
    content: Vec<Chapter>,
}

// * Convert the Article struct to a JSON string
fn convert_to_json(article: &Article) -> Result<String, serde_json::Error> {
    return serde_json::to_string(&article);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger
    env_logger::init();

    // * Create an instance of the Article struct
    let article = Article {
        title: "The Conduct of Life".to_string(),
        author: "Ralph Waldo Emmerson".to_string(),
        content: vec![
            Chapter {
                name: "Fate".to_string(),
            },
            Chapter {
                name: "Power".to_string(),
            },
            Chapter {
                name: "Wealth".to_string(),
            },
            Chapter {
                name: "Culture".to_string(),
            },
            Chapter {
                name: "Behavior".to_string(),
            },
            Chapter {
                name: "Worship".to_string(),
            },
            Chapter {
                name: "Considerations by the Way".to_string(),
            },
            Chapter {
                name: "Beauty".to_string(),
            },
            Chapter {
                name: "Illusions".to_string(),
            },
        ],
    };

    // * Serialize the article struct to a JSON string
    let json = convert_to_json(&article)?;

    let output_file = "output.json";

    // * Write the JSON string to a file with pretty formatting
    write(
        output_file,
        serde_json::to_string_pretty(&article)?.as_bytes(),
    )?;

    // * log the json string in the console
    // ! to view the log, run the program with the RUST_LOG=info environment variable
    info!("JSON string: {}", json);

    return Ok(());
}
