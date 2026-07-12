use std::{
    io::{self, Write},
    sync::Arc,
};

use langchain_rust::{
    agent::{AgentExecutor, OpenAiToolAgentBuilder},
    chain::{Chain, options::ChainCallOptions},
    llm::{OpenAI, ollama::openai::OllamaConfig},
    memory::SimpleMemory,
    prompt_args,
    tools::Tool,
};

use crate::{
    agent_model::{AgentModel, MODEL, OLLAMA_HOST, OLLAMA_PORT, SYSTEM_PROMPT},
    lc::tools::{time::Time, word_count::WordCount},
};

pub struct LangChainModel;

impl AgentModel for LangChainModel {
    async fn process(&self) {
        // ----- LLM -----
        // Ollama's OpenAI-compatible endpoint (/v1) supports real tool-calling for
        // models that advertise it (qwen3.5 does), unlike the native ollama_rs
        // client used by LangChainModel, which silently drops `functions` options.
        let ollama_config =
            OllamaConfig::new().with_api_base(format!("{OLLAMA_HOST}:{OLLAMA_PORT}/v1"));
        let llm = OpenAI::new(ollama_config).with_model(MODEL);

        // ----- Tools -----
        let tools: Vec<Arc<dyn Tool>> = vec![Arc::new(Time), Arc::new(WordCount)];

        // ----- System prompt + agent -----
        let agent = OpenAiToolAgentBuilder::new()
            .tools(&tools)
            .prefix(SYSTEM_PROMPT)
            .options(ChainCallOptions::new().with_max_tokens(10000))
            .build(llm)
            .expect("failed to build agent");

        // ----- Memory -----
        let memory = SimpleMemory::new();
        let executor = AgentExecutor::from_agent(agent).with_memory(memory.into());

        // ----- Process user input -----
        println!("Ready! Ask the agent something. It remembers the conversation.\n");

        let stdin = io::stdin();
        loop {
            print!("You: ");
            io::stdout().flush().expect("failed to flush stdout");

            let mut question = String::new();
            if stdin
                .read_line(&mut question)
                .expect("failed to read stdin")
                == 0
            {
                break; // EOF (e.g. piped input ended)
            }

            let question = question.trim();
            if question.is_empty() || question.eq_ignore_ascii_case("exit") {
                break;
            }

            match executor.invoke(prompt_args! { "input" => question }).await {
                Ok(answer) => println!("\nAnswer: {answer}\n"),
                Err(e) => println!("\nError: {e}\n"),
            }
        }
    }
}
