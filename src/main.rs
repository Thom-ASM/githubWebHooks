mod SlackReaction;
mod github;

use actix_web::{
    post,
    web::{self},
    App, HttpServer, Responder,
};

use crate::github::{GithubPrReviewState, GithubResponse};
use dotenv::dotenv;
use SlackReaction::SendableToSlack;

async fn send_update_to_slack(state: &GithubPrReviewState) -> Result<(), ()> {
    let mut args = SlackReaction::SlackReactionArgs {
        channel: dotenv::var("SLACK_CHANNEL_ID").unwrap().to_string(),
        name: SlackReaction::Reaction::white_check_mark,
        timestamp: "1657833380.803579".to_string(),
    };

    match state {
        GithubPrReviewState::approved => {
            args.update_reaction(SlackReaction::Reaction::white_check_mark);
        }
        GithubPrReviewState::changes_requested => {
            args.update_reaction(SlackReaction::Reaction::x);
        }
        _ => {
            args.update_reaction(SlackReaction::Reaction::eyes);
        }
    }

    let res = args.send_request(&args).await?;

    let text = res.text().await.unwrap();
    println!("{:#}", text);

    Ok(())
}

#[post("/github")]
async fn handle_response(payload: web::Json<GithubResponse>) -> impl Responder {
    send_update_to_slack(&payload.review.state).await.unwrap();

    format!("Hello github!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| async { "Hello World!" }))
            .service(handle_response)
    })
    .bind(("127.0.0.1", 4567))?
    .run()
    .await
}
