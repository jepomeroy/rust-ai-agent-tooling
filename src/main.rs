mod agent_model;
mod lc;

use crate::agent_model::AgentModel;
use crate::lc::langchain::LangChainModel;

#[tokio::main]
async fn main() {
    env_logger::init();
    let model = LangChainModel {};

    model.process().await;
}
