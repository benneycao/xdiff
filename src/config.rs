use std::collections::HashMap;
use reqwest::header::HeaderMap;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use tokio::fs;
use url::Url;
use anyhow::Result;


#[derive(Debug, Serialize, Deserialize)]
pub struct DiffConfig {
    #[serde(flatten)]
    pub profiles: HashMap<String, DiffProfile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffProfile {
    pub req1: RequestProfile,
    pub req2: RequestProfile,
    #[serde(skip_serializing_if = "ResponseProfile::is_empty", default)]
    pub res: ResponseProfile,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestProfile {
    #[serde(with = "http_serde::method", default)]
    pub method: Method,
    pub url: Url,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    // #[serde(skip_serializing_if = "empty_json_value", default)]
    pub params: Option<serde_json::Value>,
    #[serde(
    skip_serializing_if = "HeaderMap::is_empty",
    with = "http_serde::header_map",
    default
    )]
    pub headers: HeaderMap,
    pub body: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ResponseProfile {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_heads: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_body: Vec<String>,
}

impl ResponseProfile {
    fn is_empty(&self) -> bool {
        if self.skip_body.is_empty() && self.skip_heads.is_empty() { true } else { false }
    }

}

impl DiffConfig {
    pub async fn load_yaml(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path).await?;
        Self::from_yaml(&content)
    }
    pub fn from_yaml(content: &str) -> Result<Self> {
        Ok(serde_yaml::from_str(content)?)
    }

    pub fn get_profile(&self, name: &str) -> Option<&DiffProfile> {
        todo!()
        // self.profiles.get(name)
    }
}

impl DiffProfile {
    // pub async fn diff(&self, args: DiffArgs) -> Result<String> {
    //     let res1 = req1.send(&args).await?;
    //     let res2 = req2.send(&args).await?;
    //
    //     let text1 = res1.filter_text(&self.res).await?;
    //     let text2 = res2.filter_text(&self.res).await?;
    //     text_diff(text1, text2)
    //     todo!()
    // }
}

