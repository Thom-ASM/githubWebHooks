use std::fmt::Debug;

use async_trait::async_trait;
use dotenv::var;
use reqwest::Response;
use serde::Serialize;
use serde_json::to_string;

#[derive(Debug, Serialize)]
pub struct SlackReactionArgs {
    pub channel: String,
    pub name: Reaction,
    pub timestamp: String,
}

impl SlackReactionArgs {
    pub fn update_reaction(&mut self, new_reaction: Reaction) {
        self.name = new_reaction;
    }
}

#[async_trait(?Send)]
pub trait SendableToSlack {
    async fn send_request(&self, payload: &SlackReactionArgs) -> Result<Response, ()>;
}

#[async_trait(?Send)]
impl SendableToSlack for SlackReactionArgs {
    async fn send_request(&self, payload: &SlackReactionArgs) -> Result<Response, ()> {
        let http_client = reqwest::Client::new();

        let response = http_client
            .post("https://slack.com/api/reactions.add")
            .bearer_auth(var("SLACK_BOT_OAUTH_TOKEN").unwrap())
            .header("Content-Type", "application/json; charset=UTF-8")
            .json(payload)
            .send()
            .await;

        match response {
            Ok(res) => return Ok(res),
            Err(_) => return Err(()),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum Reaction {
    white_check_mark,
    x,
    eyes,
}
