use serde::{Deserialize, Serialize};

use super::user::User;




#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct Member {
    user: Option<User>,
    nick: Option<String>,
    permissions: Option<String>
}

impl Member {
    pub fn is_admin(&self) -> bool {
        match &self.permissions {
            Some(permissions) => {
                let num = permissions.parse::<u64>().unwrap_or(0);
                num & 8 == 8
            },
            None => false
        }
    }
}