pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ChatCompletionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<ChatCompletionRequestLogprobs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<i64>,
    /// Output-token cap for this request. An API key may set this above the agent's Max Output Tokens. The value cannot exceed the model's maximum. Requests without an API key keep the agent's Max Output Tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i64>,
    /// Chat messages in OpenAI format. Required when `prompt` is omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<ChatMessage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ChatCompletionRequestMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modalities: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prediction: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,
    /// User text to complete. Required when `messages` is omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<ChatCompletionRequestReasoningEffort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbosity: Option<ChatCompletionRequestVerbosity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ChatCompletionRequestResponseFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    /// OpenAI-compatible function tool definitions. Passed through to the model as declaration-only tools (the client executes them and returns role=tool messages).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ChatCompletionRequestToolChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl ChatCompletionRequest {
    pub fn builder() -> ChatCompletionRequestBuilder {
        <ChatCompletionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatCompletionRequestBuilder {
    audio: Option<HashMap<String, serde_json::Value>>,
    frequency_penalty: Option<f64>,
    logit_bias: Option<HashMap<String, serde_json::Value>>,
    logprobs: Option<ChatCompletionRequestLogprobs>,
    top_logprobs: Option<i64>,
    max_completion_tokens: Option<i64>,
    max_tokens: Option<i64>,
    messages: Option<Vec<ChatMessage>>,
    metadata: Option<ChatCompletionRequestMetadata>,
    modalities: Option<Vec<serde_json::Value>>,
    model: Option<String>,
    n: Option<i64>,
    parallel_tool_calls: Option<bool>,
    prediction: Option<HashMap<String, serde_json::Value>>,
    presence_penalty: Option<f64>,
    prompt: Option<String>,
    reasoning_effort: Option<ChatCompletionRequestReasoningEffort>,
    verbosity: Option<ChatCompletionRequestVerbosity>,
    response_format: Option<ChatCompletionRequestResponseFormat>,
    seed: Option<i64>,
    service_tier: Option<String>,
    stop: Option<String>,
    store: Option<bool>,
    stream: Option<bool>,
    stream_options: Option<HashMap<String, serde_json::Value>>,
    temperature: Option<f64>,
    top_p: Option<f64>,
    tools: Option<Vec<serde_json::Value>>,
    tool_choice: Option<ChatCompletionRequestToolChoice>,
    user: Option<String>,
}

impl ChatCompletionRequestBuilder {
    pub fn audio(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.audio = Some(value);
        self
    }

    pub fn frequency_penalty(mut self, value: f64) -> Self {
        self.frequency_penalty = Some(value);
        self
    }

    pub fn logit_bias(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.logit_bias = Some(value);
        self
    }

    pub fn logprobs(mut self, value: ChatCompletionRequestLogprobs) -> Self {
        self.logprobs = Some(value);
        self
    }

    pub fn top_logprobs(mut self, value: i64) -> Self {
        self.top_logprobs = Some(value);
        self
    }

    pub fn max_completion_tokens(mut self, value: i64) -> Self {
        self.max_completion_tokens = Some(value);
        self
    }

    pub fn max_tokens(mut self, value: i64) -> Self {
        self.max_tokens = Some(value);
        self
    }

    pub fn messages(mut self, value: Vec<ChatMessage>) -> Self {
        self.messages = Some(value);
        self
    }

    pub fn metadata(mut self, value: ChatCompletionRequestMetadata) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn modalities(mut self, value: Vec<serde_json::Value>) -> Self {
        self.modalities = Some(value);
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn n(mut self, value: i64) -> Self {
        self.n = Some(value);
        self
    }

    pub fn parallel_tool_calls(mut self, value: bool) -> Self {
        self.parallel_tool_calls = Some(value);
        self
    }

    pub fn prediction(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.prediction = Some(value);
        self
    }

    pub fn presence_penalty(mut self, value: f64) -> Self {
        self.presence_penalty = Some(value);
        self
    }

    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    pub fn reasoning_effort(mut self, value: ChatCompletionRequestReasoningEffort) -> Self {
        self.reasoning_effort = Some(value);
        self
    }

    pub fn verbosity(mut self, value: ChatCompletionRequestVerbosity) -> Self {
        self.verbosity = Some(value);
        self
    }

    pub fn response_format(mut self, value: ChatCompletionRequestResponseFormat) -> Self {
        self.response_format = Some(value);
        self
    }

    pub fn seed(mut self, value: i64) -> Self {
        self.seed = Some(value);
        self
    }

    pub fn service_tier(mut self, value: impl Into<String>) -> Self {
        self.service_tier = Some(value.into());
        self
    }

    pub fn stop(mut self, value: impl Into<String>) -> Self {
        self.stop = Some(value.into());
        self
    }

    pub fn store(mut self, value: bool) -> Self {
        self.store = Some(value);
        self
    }

    pub fn stream(mut self, value: bool) -> Self {
        self.stream = Some(value);
        self
    }

    pub fn stream_options(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.stream_options = Some(value);
        self
    }

    pub fn temperature(mut self, value: f64) -> Self {
        self.temperature = Some(value);
        self
    }

    pub fn top_p(mut self, value: f64) -> Self {
        self.top_p = Some(value);
        self
    }

    pub fn tools(mut self, value: Vec<serde_json::Value>) -> Self {
        self.tools = Some(value);
        self
    }

    pub fn tool_choice(mut self, value: ChatCompletionRequestToolChoice) -> Self {
        self.tool_choice = Some(value);
        self
    }

    pub fn user(mut self, value: impl Into<String>) -> Self {
        self.user = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ChatCompletionRequest`].
    pub fn build(self) -> Result<ChatCompletionRequest, BuildError> {
        Ok(ChatCompletionRequest {
            audio: self.audio,
            frequency_penalty: self.frequency_penalty,
            logit_bias: self.logit_bias,
            logprobs: self.logprobs,
            top_logprobs: self.top_logprobs,
            max_completion_tokens: self.max_completion_tokens,
            max_tokens: self.max_tokens,
            messages: self.messages,
            metadata: self.metadata,
            modalities: self.modalities,
            model: self.model,
            n: self.n,
            parallel_tool_calls: self.parallel_tool_calls,
            prediction: self.prediction,
            presence_penalty: self.presence_penalty,
            prompt: self.prompt,
            reasoning_effort: self.reasoning_effort,
            verbosity: self.verbosity,
            response_format: self.response_format,
            seed: self.seed,
            service_tier: self.service_tier,
            stop: self.stop,
            store: self.store,
            stream: self.stream,
            stream_options: self.stream_options,
            temperature: self.temperature,
            top_p: self.top_p,
            tools: self.tools,
            tool_choice: self.tool_choice,
            user: self.user,
        })
    }
}
