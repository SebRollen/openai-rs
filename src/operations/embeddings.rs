use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use vila::{Method, Request, RequestData};

#[derive(Debug, Deserialize, Serialize)]
pub enum Model {
    #[serde(rename = "text-embedding-3-large")]
    TextEmbedding3Large,
    #[serde(rename = "text-embedding-3-small")]
    TextEmbedding3Small,
    #[serde(rename = "text-embedding-ada-002")]
    TextEmbeddingAda2,
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum Input {
    One(String),
    Many(Vec<String>),
}

impl Into<Input> for &'_ str {
    fn into(self) -> Input {
        Input::One(self.to_string())
    }
}

impl Into<Input> for String {
    fn into(self) -> Input {
        Input::One(self)
    }
}

impl Into<Input> for Vec<&'_ str> {
    fn into(self) -> Input {
        let v = self.into_iter().map(|s| s.to_string()).collect();
        Input::Many(v)
    }
}

impl Into<Input> for Vec<String> {
    fn into(self) -> Input {
        Input::Many(self)
    }
}

#[derive(Default, Deserialize, Serialize)]
pub enum EncodingFormat {
    Base64,
    #[default]
    Float,
}

#[derive(Deserialize, Serialize)]
pub struct EmbeddingRequest {
    input: Input,
    model: Model,
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding_format: Option<EncodingFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dimensions: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>,
}

impl EmbeddingRequest {
    pub fn new<I>(input: I, model: Model) -> Self
    where
        I: Into<Input>,
    {
        Self {
            input: input.into(),
            model,
            dimensions: None,
            encoding_format: None,
            user: None,
        }
    }

    pub fn encoding_format(mut self, encoding_format: EncodingFormat) -> Self {
        self.encoding_format = Some(encoding_format);
        self
    }

    pub fn dimensions(mut self, dimensions: usize) -> Self {
        self.dimensions = Some(dimensions);
        self
    }

    pub fn user<T: ToString>(mut self, user: T) -> Self {
        self.user = Some(user.to_string());
        self
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Embedding {
    /// The object type, which is always "embedding".
    pub object: String,
    /// THe embedding vector consists of a list of floating-point numbers. The length of this
    /// vector varies depending on the specific model.
    pub embedding: Vec<f32>,
    /// An integer representing the index of the embedding within the list of embeddings.
    pub index: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Usage {
    pub prompt_tokens: usize,
    /// The total number of tokens used for computing the embeddings.
    pub total_tokens: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EmbeddingResponse {
    /// The object type, which is always "list".
    pub object: String,
    /// An array of embedding objects.
    pub data: Vec<Embedding>,
    /// Name of the model.
    pub model: Model,
    pub usage: Usage,
}

impl Request for EmbeddingRequest {
    type Data = Self;
    type Response = EmbeddingResponse;
    const METHOD: Method = Method::POST;

    fn endpoint(&self) -> Cow<'_, str> {
        Cow::Borrowed("v1/embeddings")
    }

    fn data(&self) -> RequestData<&Self> {
        RequestData::Json(self)
    }
}
