use std::time::Duration;
use log::{debug};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ApiError {
    message: String,
}

#[derive(Deserialize, Debug)]
pub struct OnlinePlayer {
    pub name: String,
    pub vocation: String,
    pub level: i32,
}

#[derive(Deserialize, Debug)]
pub struct World {
    pub name: String,
    pub online_players: Vec<OnlinePlayer>
}

#[derive(Deserialize, Debug)]
pub struct WorldResponse {
    pub world: World,
}

pub struct TibiaDataClient {
    client: reqwest::Client,
    base_url: String,
}

impl TibiaDataClient {
    pub(crate) fn new() -> TibiaDataClient {
        TibiaDataClient {
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(10))
                .user_agent("rust/reqwest")
                .build()
                .unwrap(),
            base_url: "https://api.tibiadata.com/v4/".into(),
        }
    }

    pub(crate) async fn get_world(&self, world: &str) -> Result<WorldResponse, Box<dyn std::error::Error>> {
        let mut url = self.base_url.to_string();
        url.push_str("/world/");
        url.push_str(world);

        let resp = self.client.get(url)
            .send()
            .await?;

        let status = resp.status();
        debug!("Status: {}", status);

        if !status.is_success() {
            let api_error: ApiError = resp.json().await?;
            return Err(api_error.message.into());
        }

        let world_response: WorldResponse = resp.json().await?;
        Ok(world_response)
    }
}
