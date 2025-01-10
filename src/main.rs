use serde::{Deserialize, Serialize};
use std::fs::write;

// * Implement the Serialize and Deserialize traits for the Paragraph struct
#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

// * Implement the Serialize and Deserialize traits for the Article struct
#[derive(Serialize, Deserialize)]
struct Article {
    title: String,
    author: String,
    content: Vec<Paragraph>,
}

// * Convert the Article struct to a JSON string
fn convert_to_json(article: Article) -> Result<String, serde_json::Error> {
    serde_json::to_string(&article)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // * Create an instance of the Article struct
    let article = Article {
        title: "The Conduct of Life".to_string(),
        author: "Ralph Waldo Emmerson".to_string(),
        content: vec![
            Paragraph {
                name: "Fate".to_string(),
            },
            Paragraph {
                name: "Power".to_string(),
            },
            Paragraph {
                name: "Wealth".to_string(),
            },
            Paragraph {
                name: "Culture".to_string(),
            },
        ],
    };

    // * Serialize the article struct to a JSON string
    let json = convert_to_json(article)?;

    let output_file = "output.json";

    // * Write the JSON string to a file
    write(output_file, json.as_bytes())?;

    println!("{}", json);

    Ok(())
}
