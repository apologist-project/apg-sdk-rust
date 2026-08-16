# Reference
## Chat
<details><summary><code>client.chat.<a href="/src/api/resources/chat/client.rs">list_chat_completions</a>(page: Option&lt;Option&lt;i64&gt;&gt;, per_page: Option&lt;Option&lt;i64&gt;&gt;, agent_id: Option&lt;Option&lt;String&gt;&gt;, channel_id: Option&lt;Option&lt;String&gt;&gt;, bible_id: Option&lt;Option&lt;String&gt;&gt;, cached: Option&lt;Option&lt;String&gt;&gt;, client: Option&lt;Option&lt;String&gt;&gt;, config_id: Option&lt;Option&lt;String&gt;&gt;, conversation_id: Option&lt;Option&lt;String&gt;&gt;, device_id: Option&lt;Option&lt;String&gt;&gt;, flagged: Option&lt;Option&lt;String&gt;&gt;, favorited: Option&lt;Option&lt;String&gt;&gt;, language: Option&lt;Option&lt;String&gt;&gt;, liked: Option&lt;Option&lt;String&gt;&gt;, session_id: Option&lt;Option&lt;String&gt;&gt;, user_id: Option&lt;Option&lt;String&gt;&gt;, min_timestamp: Option&lt;Option&lt;String&gt;&gt;, max_timestamp: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListChatCompletionsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of chat completions (prompts) for the agent, with applied tags expanded as { id, name } and share metadata.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apologist::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApologistAgentClient::new(config).expect("Failed to build client");
    client
        .chat
        .list_chat_completions(
            &ListChatCompletionsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<i64>` 
    
</dd>
</dl>

<dl>
<dd>

**per_page:** `Option<i64>` — Results per page (clamped to 100).
    
</dd>
</dl>

<dl>
<dd>

**agent_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**channel_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**bible_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**cached:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**client:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**config_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**conversation_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**device_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**flagged:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**favorited:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**language:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**liked:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**min_timestamp:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**max_timestamp:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.chat.<a href="/src/api/resources/chat/client.rs">create_chat_completion</a>(request: ChatCompletionRequest) -> Result&lt;ChatCompletionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates a chat completion using the agent's configured model. Supports both streaming and non-streaming responses.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apologist::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApologistAgentClient::new(config).expect("Failed to build client");
    client
        .chat
        .create_chat_completion(
            &ChatCompletionRequest::Unknown(serde_json::json!({"key":"value"})),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.chat.<a href="/src/api/resources/chat/client.rs">like_completion</a>(id: String, request: LikeRequest) -> Result&lt;SuccessResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates the like status of a specific chat completion
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apologist::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApologistAgentClient::new(config).expect("Failed to build client");
    client
        .chat
        .like_completion(&"id".to_string(), &LikeRequest { liked: true }, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The ID of the chat completion
    
</dd>
</dl>

<dl>
<dd>

**liked:** `bool` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.chat.<a href="/src/api/resources/chat/client.rs">flag_completion</a>(id: String, request: FlagRequest) -> Result&lt;SuccessResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates the flagged status of a specific chat completion
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apologist::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApologistAgentClient::new(config).expect("Failed to build client");
    client
        .chat
        .flag_completion(&"id".to_string(), &FlagRequest { flagged: true }, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The ID of the chat completion
    
</dd>
</dl>

<dl>
<dd>

**flagged:** `bool` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.chat.<a href="/src/api/resources/chat/client.rs">feedback_completion</a>(id: String, request: FeedbackRequest) -> Result&lt;SuccessResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Adds user feedback to a specific chat completion
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apologist::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApologistAgentClient::new(config).expect("Failed to build client");
    client
        .chat
        .feedback_completion(
            &"id".to_string(),
            &FeedbackRequest {
                feedback: "feedback".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The ID of the chat completion
    
</dd>
</dl>

<dl>
<dd>

**feedback:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.chat.<a href="/src/api/resources/chat/client.rs">share_completion</a>(id: String, request: ShareRequest) -> Result&lt;SuccessResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates a share record for a specific chat completion
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apologist::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApologistAgentClient::new(config).expect("Failed to build client");
    client
        .chat
        .share_completion(
            &"id".to_string(),
            &ShareRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The ID of the chat completion
    
</dd>
</dl>

<dl>
<dd>

**conversation_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**session_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<Option<String>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.chat.<a href="/src/api/resources/chat/client.rs">get_chat_completion</a>(id: String) -> Result&lt;GetChatCompletionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a single chat completion (prompt) by numeric id or UUID, including applied tags, guardrail/cta metadata, share metadata, and automation results.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apologist::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApologistAgentClient::new(config).expect("Failed to build client");
    client
        .chat
        .get_chat_completion(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The numeric id or UUID of the chat completion
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Corpus
<details><summary><code>client.corpus.<a href="/src/api/resources/corpus/client.rs">search_corpus</a>(request: CorpusSearchRequest) -> Result&lt;SearchCorpusResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Performs a semantic search across the agent's corpus of knowledge
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apologist::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApologistAgentClient::new(config).expect("Failed to build client");
    client
        .corpus
        .search_corpus(
            &CorpusSearchRequest {
                query: "query".to_string(),
                prompt_id: None,
                limit: None,
                filters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**query:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**prompt_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**filters:** `Option<Option<CorpusSearchRequestFilters>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.corpus.<a href="/src/api/resources/corpus/client.rs">log_corpus_view</a>(model: String, id: String, request: ViewRequest) -> Result&lt;SuccessResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Records that a user viewed a specific corpus item
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apologist::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApologistAgentClient::new(config).expect("Failed to build client");
    client
        .corpus
        .log_corpus_view(
            &"model".to_string(),
            &"id".to_string(),
            &ViewRequest {
                prompt_id: "prompt_id".to_string(),
                user_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**model:** `String` — The model type (e.g., 'source')
    
</dd>
</dl>

<dl>
<dd>

**id:** `String` — The ID of the corpus item
    
</dd>
</dl>

<dl>
<dd>

**prompt_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<Option<String>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.corpus.<a href="/src/api/resources/corpus/client.rs">log_corpus_impression</a>(model: String, id: String, request: ImpressionRequest) -> Result&lt;SuccessResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Records that a corpus item was shown to a user
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apologist::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApologistAgentClient::new(config).expect("Failed to build client");
    client
        .corpus
        .log_corpus_impression(
            &"model".to_string(),
            &"id".to_string(),
            &ImpressionRequest {
                prompt_id: "prompt_id".to_string(),
                user_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**model:** `String` — The model type (e.g., 'source')
    
</dd>
</dl>

<dl>
<dd>

**id:** `String` — The ID of the corpus item
    
</dd>
</dl>

<dl>
<dd>

**prompt_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<Option<String>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.corpus.<a href="/src/api/resources/corpus/client.rs">log_corpus_referral_redirect</a>(model: String, id: String, prompt_id: Option&lt;String&gt;, user_id: Option&lt;Option&lt;String&gt;&gt;, url: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;SuccessResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Records a referral for a corpus item and, when a `url` is supplied, issues a 302 redirect to it. Without a `url`, responds with a success message. Requires either the search API entitlement or a signed `browser_key` cookie.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apologist::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApologistAgentClient::new(config).expect("Failed to build client");
    client
        .corpus
        .log_corpus_referral_redirect(
            &"model".to_string(),
            &"id".to_string(),
            &LogCorpusReferralRedirectQueryRequest {
                prompt_id: "prompt_id".to_string(),
                user_id: None,
                url: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**model:** `String` — The model type (e.g., 'source')
    
</dd>
</dl>

<dl>
<dd>

**id:** `String` — The numeric ID of the corpus item
    
</dd>
</dl>

<dl>
<dd>

**prompt_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**url:** `Option<String>` — URL-encoded destination to redirect to after logging the referral.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.corpus.<a href="/src/api/resources/corpus/client.rs">log_corpus_referral</a>(model: String, id: String, request: ReferralRequest) -> Result&lt;SuccessResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Records that a user was referred to a corpus item
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apologist::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApologistAgentClient::new(config).expect("Failed to build client");
    client
        .corpus
        .log_corpus_referral(
            &"model".to_string(),
            &"id".to_string(),
            &ReferralRequest {
                prompt_id: "prompt_id".to_string(),
                user_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**model:** `String` — The model type (e.g., 'source')
    
</dd>
</dl>

<dl>
<dd>

**id:** `String` — The ID of the corpus item
    
</dd>
</dl>

<dl>
<dd>

**prompt_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<Option<String>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Evaluators
<details><summary><code>client.evaluators.<a href="/src/api/resources/evaluators/client.rs">list_evaluations</a>(id: String, page: Option&lt;Option&lt;i64&gt;&gt;, per_page: Option&lt;Option&lt;i64&gt;&gt;, min_timestamp: Option&lt;Option&lt;String&gt;&gt;, max_timestamp: Option&lt;Option&lt;String&gt;&gt;, min_duration: Option&lt;Option&lt;String&gt;&gt;, max_duration: Option&lt;Option&lt;String&gt;&gt;, min_score: Option&lt;Option&lt;String&gt;&gt;, max_score: Option&lt;Option&lt;String&gt;&gt;, passed: Option&lt;Option&lt;String&gt;&gt;, benchmark: Option&lt;Option&lt;String&gt;&gt;, benchmark_run_id: Option&lt;Option&lt;String&gt;&gt;, benchmark_question_id: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListEvaluationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of evaluations for the evaluator, scoped to the requesting agent.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apologist::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApologistAgentClient::new(config).expect("Failed to build client");
    client
        .evaluators
        .list_evaluations(
            &"id".to_string(),
            &ListEvaluationsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The ID or key of the evaluator
    
</dd>
</dl>

<dl>
<dd>

**page:** `Option<i64>` 
    
</dd>
</dl>

<dl>
<dd>

**per_page:** `Option<i64>` — Results per page (clamped to 100).
    
</dd>
</dl>

<dl>
<dd>

**min_timestamp:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**max_timestamp:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**min_duration:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**max_duration:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**min_score:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**max_score:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**passed:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**benchmark:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**benchmark_run_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**benchmark_question_id:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.evaluators.<a href="/src/api/resources/evaluators/client.rs">evaluate_content</a>(id: String, request: EvaluatorRequest) -> Result&lt;EvaluateContentResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Runs an evaluation on the provided content using the specified evaluator
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apologist::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApologistAgentClient::new(config).expect("Failed to build client");
    client
        .evaluators
        .evaluate_content(
            &"id".to_string(),
            &EvaluatorRequest {
                content: EvaluatorRequestContent::String("content".to_string()),
                frequency_penalty: None,
                confidence_threshold: None,
                model: None,
                presence_penalty: None,
                reasoning_effort: None,
                verbosity: None,
                temperature: None,
                top_p: None,
                variables: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The ID or key of the evaluator
    
</dd>
</dl>

<dl>
<dd>

**frequency_penalty:** `Option<Option<f64>>` 
    
</dd>
</dl>

<dl>
<dd>

**confidence_threshold:** `Option<Option<f64>>` 
    
</dd>
</dl>

<dl>
<dd>

**content:** `EvaluatorRequestContent` 
    
</dd>
</dl>

<dl>
<dd>

**model:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**presence_penalty:** `Option<Option<f64>>` 
    
</dd>
</dl>

<dl>
<dd>

**reasoning_effort:** `Option<Option<EvaluatorRequestReasoningEffort>>` 
    
</dd>
</dl>

<dl>
<dd>

**verbosity:** `Option<Option<EvaluatorRequestVerbosity>>` 
    
</dd>
</dl>

<dl>
<dd>

**temperature:** `Option<Option<f64>>` 
    
</dd>
</dl>

<dl>
<dd>

**top_p:** `Option<Option<f64>>` 
    
</dd>
</dl>

<dl>
<dd>

**variables:** `Option<Option<std::collections::HashMap<String, Option<String>>>>` — Flat string key/value pairs substituted into `{key}` placeholders in the evaluator prompt. Reserved keys (`options`, `option_descriptions`, `criteria`) cannot be overridden. Not persisted; omitted from the response.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.evaluators.<a href="/src/api/resources/evaluators/client.rs">get_evaluation</a>(id: String, evaluation_id: String) -> Result&lt;GetEvaluationResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a single evaluation for the evaluator, scoped to the requesting agent.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apologist::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApologistAgentClient::new(config).expect("Failed to build client");
    client
        .evaluators
        .get_evaluation(&"id".to_string(), &"evaluationId".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The id or key of the evaluator
    
</dd>
</dl>

<dl>
<dd>

**evaluation_id:** `String` — The id or UUID of the evaluation
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## CTAs
<details><summary><code>client.ct_as.<a href="/src/api/resources/ct_as/client.rs">match_ctas</a>(request: CtaMatchRequest) -> Result&lt;MatchCtasResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Finds matching CTAs based on conversation context, user, session, device, or messages
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apologist::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApologistAgentClient::new(config).expect("Failed to build client");
    client
        .ct_as
        .match_ctas(
            &CtaMatchRequest::Unknown(serde_json::json!({"key":"value"})),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ct_as.<a href="/src/api/resources/ct_as/client.rs">log_cta_click</a>(id: String, request: CtaClickRequest) -> Result&lt;SuccessResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Records that a user clicked on a specific CTA
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apologist::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApologistAgentClient::new(config).expect("Failed to build client");
    client
        .ct_as
        .log_cta_click(
            &"id".to_string(),
            &CtaClickRequest {
                prompt_id: "prompt_id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The ID of the CTA
    
</dd>
</dl>

<dl>
<dd>

**prompt_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Users
<details><summary><code>client.users.<a href="/src/api/resources/users/client.rs">list_users</a>(page: Option&lt;Option&lt;i64&gt;&gt;, per_page: Option&lt;Option&lt;i64&gt;&gt;, external_id: Option&lt;Option&lt;String&gt;&gt;, tags: Option&lt;Option&lt;String&gt;&gt;, responder_id: Option&lt;Option&lt;String&gt;&gt;, min_timestamp: Option&lt;Option&lt;String&gt;&gt;, max_timestamp: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListUsersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of users for the agent's team, with applied tags expanded as { id, name } and the persisted responder id.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apologist::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApologistAgentClient::new(config).expect("Failed to build client");
    client
        .users
        .list_users(
            &ListUsersQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<i64>` 
    
</dd>
</dl>

<dl>
<dd>

**per_page:** `Option<i64>` — Results per page (clamped to 100).
    
</dd>
</dl>

<dl>
<dd>

**external_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**tags:** `Option<String>` — Comma-separated tag ids.
    
</dd>
</dl>

<dl>
<dd>

**responder_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**min_timestamp:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**max_timestamp:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.users.<a href="/src/api/resources/users/client.rs">list_user_flags</a>(page: Option&lt;Option&lt;i64&gt;&gt;, per_page: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListUserFlagsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of user flag definitions for the agent's team (all columns from user_flags), ordered by id ascending.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apologist::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApologistAgentClient::new(config).expect("Failed to build client");
    client
        .users
        .list_user_flags(
            &ListUserFlagsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page:** `Option<i64>` 
    
</dd>
</dl>

<dl>
<dd>

**per_page:** `Option<i64>` — Results per page (clamped to 100).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.users.<a href="/src/api/resources/users/client.rs">get_user</a>(user_id: String) -> Result&lt;GetUserResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a single user by external id or internal id, with expanded tags and the persisted responder for the agent.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apologist::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApologistAgentClient::new(config).expect("Failed to build client");
    client.users.get_user(&"user_id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**user_id:** `String` — The user's external id or internal id
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.users.<a href="/src/api/resources/users/client.rs">update_user</a>(user_id: String, request: UserUpdateRequest) -> Result&lt;UpdateUserResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates a user's external_id and/or tags and upserts the persisted responder for the agent. Only provided fields are changed.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apologist::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApologistAgentClient::new(config).expect("Failed to build client");
    client
        .users
        .update_user(
            &"user_id".to_string(),
            &UserUpdateRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**user_id:** `String` — The user's external id or internal id
    
</dd>
</dl>

<dl>
<dd>

**external_id:** `Option<Option<String>>` — Your external identifier for the user.
    
</dd>
</dl>

<dl>
<dd>

**tags:** `Option<Vec<UserUpdateRequestTagsItem>>` — Applied tags as a mix of existing tag ids and/or default-language tag names. Unknown ids or names are rejected. Tags are mirror-owned and never created here.
    
</dd>
</dl>

<dl>
<dd>

**responder_id:** `Option<i64>` — Responder to persist for this user on the requesting agent. Must be active on the agent.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Benchmarks
<details><summary><code>client.benchmarks.<a href="/src/api/resources/benchmarks/client.rs">list_benchmark_runs</a>(id: String, page: Option&lt;Option&lt;i64&gt;&gt;, per_page: Option&lt;Option&lt;i64&gt;&gt;, min_timestamp: Option&lt;Option&lt;String&gt;&gt;, max_timestamp: Option&lt;Option&lt;String&gt;&gt;, min_duration: Option&lt;Option&lt;String&gt;&gt;, max_duration: Option&lt;Option&lt;String&gt;&gt;, min_score: Option&lt;Option&lt;String&gt;&gt;, max_score: Option&lt;Option&lt;String&gt;&gt;, passed: Option&lt;Option&lt;String&gt;&gt;, min_responses: Option&lt;Option&lt;String&gt;&gt;, max_responses: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListBenchmarkRunsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of runs for a benchmark, scoped to the requesting agent. Each run carries nested evaluators, questions, and a flat evaluations array.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apologist::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApologistAgentClient::new(config).expect("Failed to build client");
    client
        .benchmarks
        .list_benchmark_runs(
            &"id".to_string(),
            &ListBenchmarkRunsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The id or key of the benchmark
    
</dd>
</dl>

<dl>
<dd>

**page:** `Option<i64>` 
    
</dd>
</dl>

<dl>
<dd>

**per_page:** `Option<i64>` — Results per page (clamped to 100).
    
</dd>
</dl>

<dl>
<dd>

**min_timestamp:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**max_timestamp:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**min_duration:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**max_duration:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**min_score:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**max_score:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**passed:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**min_responses:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**max_responses:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.benchmarks.<a href="/src/api/resources/benchmarks/client.rs">run_benchmark</a>(id: String, request: BenchmarkRunRequest) -> Result&lt;std::collections::HashMap&lt;String, serde_json::Value&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Executes a benchmark run and returns the aggregated result with nested evaluators, questions, and a flat evaluations array.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apologist::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApologistAgentClient::new(config).expect("Failed to build client");
    client
        .benchmarks
        .run_benchmark(
            &"id".to_string(),
            &BenchmarkRunRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The id or key of the benchmark
    
</dd>
</dl>

<dl>
<dd>

**content:** `Option<BenchmarkRunRequestContent>` — Content to evaluate. Required when `source_id` is supplied.
    
</dd>
</dl>

<dl>
<dd>

**completion_id:** `Option<Option<String>>` — Completion UUID whose stored response should be evaluated.
    
</dd>
</dl>

<dl>
<dd>

**source_id:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**model:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**num_responses:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**use_question_variants:** `Option<Option<bool>>` 
    
</dd>
</dl>

<dl>
<dd>

**reasoning_effort:** `Option<Option<BenchmarkRunRequestReasoningEffort>>` 
    
</dd>
</dl>

<dl>
<dd>

**verbosity:** `Option<Option<BenchmarkRunRequestVerbosity>>` 
    
</dd>
</dl>

<dl>
<dd>

**score_threshold:** `Option<Option<f64>>` 
    
</dd>
</dl>

<dl>
<dd>

**value_threshold:** `Option<Option<f64>>` 
    
</dd>
</dl>

<dl>
<dd>

**temperature:** `Option<Option<f64>>` 
    
</dd>
</dl>

<dl>
<dd>

**top_p:** `Option<Option<f64>>` 
    
</dd>
</dl>

<dl>
<dd>

**frequency_penalty:** `Option<Option<f64>>` 
    
</dd>
</dl>

<dl>
<dd>

**presence_penalty:** `Option<Option<f64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.benchmarks.<a href="/src/api/resources/benchmarks/client.rs">get_benchmark_run</a>(id: String, run_id: String) -> Result&lt;GetBenchmarkRunResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a single benchmark run by id or UUID, scoped to the requesting agent, including nested evaluators, questions, and evaluations.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apologist::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApologistAgentClient::new(config).expect("Failed to build client");
    client
        .benchmarks
        .get_benchmark_run(&"id".to_string(), &"runId".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The id or key of the benchmark
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `String` — The id or UUID of the run
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Channels
<details><summary><code>client.channels.<a href="/src/api/resources/channels/client.rs">get_discord_channel_status</a>(id: String) -> Result&lt;GetDiscordChannelStatusResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns the status of the Discord channel. Used as a lightweight health/verification endpoint.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apologist::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApologistAgentClient::new(config).expect("Failed to build client");
    client
        .channels
        .get_discord_channel_status(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The channel id
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="/src/api/resources/channels/client.rs">receive_discord_interaction</a>(id: String, request: std::collections::HashMap&lt;String, serde_json::Value&gt;) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Receives Discord interaction callbacks for the channel. Requests are verified via Ed25519 signature headers; unsigned or invalid requests are rejected. Payload shape is defined by Discord.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apologist::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApologistAgentClient::new(config).expect("Failed to build client");
    client
        .channels
        .receive_discord_interaction(
            &"id".to_string(),
            &HashMap::from([("key".to_string(), serde_json::json!("value"))]),
            Some(
                RequestOptions::new()
                    .additional_header("x-signature-ed25519", "x-signature-ed25519")
                    .additional_header("x-signature-timestamp", "x-signature-timestamp"),
            ),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The channel id
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="/src/api/resources/channels/client.rs">get_line_channel_status</a>(id: String) -> Result&lt;GetLineChannelStatusResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns the status of the LINE channel. Used as a lightweight health/verification endpoint.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apologist::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApologistAgentClient::new(config).expect("Failed to build client");
    client
        .channels
        .get_line_channel_status(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The channel id
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="/src/api/resources/channels/client.rs">receive_line_webhook</a>(id: String, request: std::collections::HashMap&lt;String, serde_json::Value&gt;) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Receives LINE Messaging API webhook events for the channel. Requests are verified via the `x-line-signature` HMAC-SHA256 (Base64) header using the channel secret unless an `api_key` is present. Payload shape is defined by LINE. The route acknowledges quickly and processes text `message` and `follow` events asynchronously.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apologist::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApologistAgentClient::new(config).expect("Failed to build client");
    client
        .channels
        .receive_line_webhook(
            &"id".to_string(),
            &HashMap::from([("key".to_string(), serde_json::json!("value"))]),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The channel id
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="/src/api/resources/channels/client.rs">verify_facebook_webhook</a>(id: String, hub_mode: Option&lt;VerifyFacebookWebhookRequestHubMode&gt;, hub_verify_token: Option&lt;String&gt;, hub_challenge: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;String, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Handles the Meta webhook verification handshake, echoing `hub.challenge` when `hub.verify_token` matches the channel's configured token.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apologist::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApologistAgentClient::new(config).expect("Failed to build client");
    client
        .channels
        .verify_facebook_webhook(
            &"id".to_string(),
            &VerifyFacebookWebhookQueryRequest {
                hub_mode: VerifyFacebookWebhookRequestHubMode::Subscribe,
                hub_verify_token: "hub.verify_token".to_string(),
                hub_challenge: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The channel id
    
</dd>
</dl>

<dl>
<dd>

**hub_mode:** `VerifyFacebookWebhookRequestHubMode` 
    
</dd>
</dl>

<dl>
<dd>

**hub_verify_token:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**hub_challenge:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="/src/api/resources/channels/client.rs">receive_facebook_message</a>(id: String, request: std::collections::HashMap&lt;String, serde_json::Value&gt;) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Receives Facebook/Messenger (and Instagram-style) message events for the channel. Payload shape is defined by Meta.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apologist::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApologistAgentClient::new(config).expect("Failed to build client");
    client
        .channels
        .receive_facebook_message(
            &"id".to_string(),
            &HashMap::from([("key".to_string(), serde_json::json!("value"))]),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The channel id
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="/src/api/resources/channels/client.rs">get_instagram_privacy_policy</a>(id: String) -> Result&lt;String, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a static HTML privacy policy page for the Instagram integration.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apologist::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApologistAgentClient::new(config).expect("Failed to build client");
    client
        .channels
        .get_instagram_privacy_policy(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The channel id
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="/src/api/resources/channels/client.rs">receive_telegram_update</a>(id: String, request: std::collections::HashMap&lt;String, serde_json::Value&gt;) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Receives Telegram bot update events for the channel. Non-message updates are acknowledged and ignored. Payload shape is defined by Telegram.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apologist::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApologistAgentClient::new(config).expect("Failed to build client");
    client
        .channels
        .receive_telegram_update(
            &"id".to_string(),
            &HashMap::from([("key".to_string(), serde_json::json!("value"))]),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The channel id
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.channels.<a href="/src/api/resources/channels/client.rs">receive_twilio_message</a>(id: String, request: ReceiveTwilioMessageRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Receives inbound Twilio messages for the channel as form-encoded data. Payload fields are defined by Twilio.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apologist::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApologistAgentClient::new(config).expect("Failed to build client");
    client
        .channels
        .receive_twilio_message(
            &"id".to_string(),
            &ReceiveTwilioMessageRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The channel id
    
</dd>
</dl>

<dl>
<dd>

**from:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**body:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Shares
<details><summary><code>client.shares.<a href="/src/api/resources/shares/client.rs">get_shared_messages</a>(token: String) -> Result&lt;GetSharedMessagesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Public, unauthenticated read of the messages behind a share token. The token is the bearer capability and enforces tenant isolation against the host agent. An empty or invalid token yields an empty messages array.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use apologist::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApologistAgentClient::new(config).expect("Failed to build client");
    client
        .shares
        .get_shared_messages(&"token".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**token:** `String` — The share token
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

