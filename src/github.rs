use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GithubResponse {
    pub action: GithubPrActions,
    pub review: GithubPr,
}

#[derive(Deserialize, Debug)]
pub enum GithubPrActions {
    closed,
    open,
    submitted,
    dismissed,
    edited,
    created,
    deleted,
    review_requested,
    reopened,
}

#[derive(Deserialize, Debug)]
pub enum GithubPrReviewState {
    approved,
    comment,
    changes_requested,
}

#[derive(Debug, Deserialize)]
pub struct GithubPr {
    pub state: GithubPrReviewState,
}
