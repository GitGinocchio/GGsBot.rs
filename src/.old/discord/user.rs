use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct User {
    id: String,
    username: String,
    discriminator: String,
}