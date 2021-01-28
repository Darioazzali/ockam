use serde::{Deserialize, Serialize};

mod create_key;
pub use create_key::*;
mod rotate_key;
pub use rotate_key::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ProfileChangeType {
    CreateKey(CreateKeyChange),
    RotateKey(RotateKeyChange),
}
