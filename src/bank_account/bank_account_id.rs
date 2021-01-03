#[derive(Debug, Clone)]
pub struct BankAccountId {
    bank_account_id: u32,
}

impl BankAccountId {
    pub fn new(bank_account_id: u32) -> Self {
        Self { bank_account_id }
    }
}
