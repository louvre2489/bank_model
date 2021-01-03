#[derive(Debug, Clone)]
pub struct UserAccountId {
    user_account_id: u32,
}

impl UserAccountId {
    pub fn new(user_account_id: u32) -> Self {
        Self { user_account_id }
    }
}
