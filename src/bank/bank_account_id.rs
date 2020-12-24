#[derive(Debug, Clone)]
pub struct BankAccountId {
    bank_account_id: i32,
}

impl BankAccountId {
    pub fn new(bank_account_id: i32) -> Self {
        Self { bank_account_id }
    }
}
