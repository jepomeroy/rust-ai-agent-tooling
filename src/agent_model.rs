// Ollama chat model. Must support tool calling. Built from ./Modelfile:
//   OLLAMA_HOST=http://terra:11434 ollama create qwen3.5-agent -f Modelfile
pub(crate) const MODEL: &str = "qwen3.5:4b";

// Remote Ollama server. Host needs a scheme (http/https); port is separate.
// Set to "http://localhost" / 11434 to use a local server instead.
pub(crate) const OLLAMA_HOST: &str = "http://terra";
pub(crate) const OLLAMA_PORT: u16 = 11434;

pub(crate) const SYSTEM_PROMPT: &str = "You are a helpful assistant with access to tools for getting the \
    current time and counting words in text. Use tools when the user's request needs one. If \
    the question doesn't need a tool, answer directly. If a tool returns an error, explain the \
    error plainly.";

pub(crate) trait AgentModel {
    async fn process(&self);
}
