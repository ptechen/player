use std::error::Error;
use std::fmt::Debug;
use gloo_net::http::Request;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use gloo_storage::Storage;

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct HttpRequest;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct HttpResponse<R> {
    pub success: bool,
    pub code: String,
    pub msg: String,
    pub data: Option<R>,
    pub trace_id: String,
}

impl HttpRequest {
    pub async fn get<T, R: DeserializeOwned>(
        body: T,
        url: &str,
    ) -> Result<HttpResponse<R>, Box<dyn Error>>
        where T: Into<JsValue>
    {
        let token:String = gloo_storage::SessionStorage::get("current_user_token").unwrap_or_default();
        let res = Request::get(url)
            .header("Content-Type", "application/json")
            .header("authorization", token.as_str())
            .body(body)?
            .send()
            .await?
            .json::<HttpResponse<R>>()
            .await?;
        Ok(res)
    }

    pub async fn get_text<T>(
        body: T,
        url: &str,
    ) -> Result<String, Box<dyn Error>>
        where T: Into<JsValue>
    {
        let token:String = gloo_storage::SessionStorage::get("current_user_token").unwrap_or_default();
        let res = Request::get(url)
            .header("Content-Type", "application/json")
            .header("authorization", token.as_str())
            .body(body)?
            .send()
            .await?
            .text()
            .await?;
        Ok(res)
    }

    pub async fn post<T, R: DeserializeOwned>(
        body: T,
        url: &str,
    ) -> Result<HttpResponse<R>, Box<dyn Error>>
        where T: Into<JsValue>
    {
        let token:String = gloo_storage::SessionStorage::get("current_user_token").unwrap_or_default();
        let res = Request::post(url)
            .header("Content-Type", "application/json")
            .header("authorization", token.as_str())
            .body(body)?
            .send()
            .await?
            .json::<HttpResponse<R>>()
            .await?;
        Ok(res)
    }

    pub async fn post_custom<T, R: DeserializeOwned>(
        body: T,
        url: &str,
    ) -> Result<R, Box<dyn Error>>
        where T: Into<JsValue>
    {
        let token:String = gloo_storage::SessionStorage::get("current_user_token").unwrap_or_default();
        let res = Request::post(url)
            .header("Content-Type", "application/json")
            .header("authorization", token.as_str())
            .body(body)?
            .send()
            .await?
            .json::<R>()
            .await?;
        Ok(res)
    }
}