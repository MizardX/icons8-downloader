use crate::core::app::IconPack;

use super::app::App;
use reqwest::StatusCode;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize)]
pub struct PlatformResponse {
    docs: Vec<Platform>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Platform {
    api_code: String,
    title: String,
}

const ICONS8_PUBLIC_ENDPOINT: &str = "https://api-icons.icons8.com/publicApi";
const ICONS8_API_TOKEN: &str = "Po1qMOE_dS4gNo3sD4KqN";

pub async fn fetch_icon_packs(app: &Arc<Mutex<App>>) {
    let req = reqwest::get(format!(
        "{ICONS8_PUBLIC_ENDPOINT}/platforms?token={ICONS8_API_TOKEN}"
    ))
    .await
    .expect("Unable to retrieve icon packs");

    if StatusCode::is_success(&req.status()) {
        let resp_text = req.text().await.expect("Unable to retrieve icon packs");
        let resp_json: PlatformResponse = serde_json::from_str(&resp_text).unwrap();
        let mut app = app.lock().await;

        for icon_pack in resp_json.docs {
            app.icon_packs
                .list
                .push(IconPack::new(icon_pack.title, icon_pack.api_code))
        }

        return;
    }

    panic!("Unable to retrieve icon packs, check your api-token.")
}
