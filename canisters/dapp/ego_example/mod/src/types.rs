use candid::CandidType;
use serde::{Deserialize, Serialize};


#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct ExampleState {}

impl Default for ExampleState{
    fn default() -> Self {
        ExampleState{}
    }
}
