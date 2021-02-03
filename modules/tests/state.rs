use serde::Deserialize;
use std::fmt::Debug;

#[derive(Debug, Clone, Deserialize)]
pub struct State {
    pub action: Action,

    #[serde(alias = "actionOutcome")]
    pub action_outcome: ActionOutcome,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Action {
    #[serde(alias = "type")]
    pub action_type: ActionType,

    #[serde(alias = "chainId")]
    pub chain_id: Option<String>,

    #[serde(alias = "clientId")]
    pub client_id: Option<u64>,

    pub height: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum ActionType {
    Null,
    CreateClient,
    UpdateClient,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum ActionOutcome {
    Null,
    CreateOK,
    UpdateOK,
    UpdateClientNotFound,
    UpdateHeightVerificationFailure,
}