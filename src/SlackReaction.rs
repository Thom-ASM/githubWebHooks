use async_trait::async_trait;
use awc::{self, Client};
use dotenv::var;
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
    fn serialize_self(&self) -> Result<String, serde_json::Error>;
    async fn send_request(&self, payload: &String) -> Result<(), ()>;
}

#[async_trait(?Send)]
impl SendableToSlack for SlackReactionArgs {
    fn serialize_self(&self) -> Result<String, serde_json::Error> {
        to_string(&self)
    }

    async fn send_request(&self, payload: &String) -> Result<(), ()> {
        let http_client = Client::new();
        let response = http_client
            .post("https://slack.com/api/reactions.add")
            .bearer_auth(var("SLACK_BOT_OAUTH_TOKEN").unwrap())
            .send_json(payload)
            .await;

        match response {
            Ok(res) => println!("response {:?}", res),
            Err(err) => println!("Error: {:?}", err),
        }

        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub enum Reaction {
    white_check_mark,
    x,
    eyes,
}
