use crate::bank::MoneyError;

#[derive(Debug, Clone)]
pub struct Money {
    balance: u32,
}

impl Money {
    pub fn new(balance: u32) -> Self {
        Self { balance }
    }

    pub fn add(&self, amount: Money) -> Result<Self, MoneyError> {
        let balance = self.balance + amount.balance;
        Ok(Self { balance })
    }

    pub fn subtract(&self, amount: Money) -> Result<Self, MoneyError> {
        let balance = self.balance - amount.balance;
        Ok(Self { balance })
    }
}
