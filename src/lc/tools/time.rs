use std::error::Error;

use async_trait::async_trait;
use chrono::Local;
use langchain_rust::tools::Tool;
use serde_json::Value;

pub(crate) struct Time;

#[async_trait]
impl Tool for Time {
    #[doc = " Returns the name of the tool."]
    fn name(&self) -> String {
        "time".to_string()
    }

    #[doc = " Provides a description of what the tool does and when to use it."]
    fn description(&self) -> String {
        r#"Return the current local date and time. 
        Use this when the user asks what time or date it is."#
            .to_string()
    }

    async fn run(&self, _input: Value) -> Result<String, Box<dyn Error>> {
        let now = Local::now();
        Ok(now.format("%Y-%m-%d %H:%M:%S").to_string())
    }
}
