use anyhow::Result;

use crate::{create_action::CreateAction, update_action::UpdateAction};

pub enum ApplyAction {
    Create(CreateAction),
    Update(UpdateAction),
}

impl ApplyAction {
    pub fn execute(&self) -> Result<()> {
        match self {
            Self::Create(action) => action.execute(),
            Self::Update(action) => action.execute(),
        }
    }
}
