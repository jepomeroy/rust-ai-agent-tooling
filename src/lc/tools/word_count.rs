use async_trait::async_trait;
use serde_json::Value;
use std::error::Error;

use langchain_rust::tools::Tool;

pub(crate) struct WordCount;

#[async_trait]
impl Tool for WordCount {
    fn name(&self) -> String {
        "word_count".to_string()
    }

    fn description(&self) -> String {
        r#"Count the number of words in a piece of text.
    Use this when the user asks how long a piece of writing is,
    or asks you to count the words in something they've shared.
    Returns the word count as an integer.
    "#
        .to_string()
    }

    async fn run(&self, input: Value) -> Result<String, Box<dyn Error>> {
        println!("{:?}", input);
        let text = input.as_str().unwrap_or_default();
        Ok(text.split_whitespace().count().to_string())
    }
}
