use crate::model::{demographics::Demographics, state::State};

pub mod demographics;
pub mod state;

#[derive(Debug)]
pub struct Economy {
    pub state: State,
    pub demographics: Demographics,
}
