use wasm_bindgen::prelude::*;
use reqwest::Client;
use wasm_bindgen_futures::future_to_promise;
use js_sys::Promise;
use serde::Deserialize;

#[wasm_bindgen]
pub fn fetch_data() -> Promise {
    future_to_promise(async {
        let client = Client::new();
        let resp = client.get("https://api.github.com/repos/rust-lang/rust")
            .header("User-Agent", "request")
            .send()
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?
            .json::<Repo>()
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(JsValue::from_serde(&resp).unwrap())
    })
}

#[derive(Deserialize)]
struct Repo {
    full_name: String,
    description: Option<String>,
}
