use crate::bank_account::BankAccountId;
use crate::bank_account::UserAccountId;
use crate::money::Money;
use crate::money::MoneyError;

#[derive(Debug, Clone)]
pub struct BankAccount {
    id: BankAccountId,
    user_account_id: UserAccountId,
    balance: Money,
}

impl BankAccount {
    pub fn new(id: BankAccountId, user_account_id: UserAccountId, balance: Money) -> Self {
        Self {
            id,
            user_account_id,
            balance,
        }
    }

    /// 残高の参照
    pub(crate) fn balance(&self) -> &Money {
        &self.balance
    }

    /// 口座への入金
    pub fn deposit(mut self, amount: Money) -> Result<BankAccount, MoneyError> {
        self.balance = self.balance.add(amount)?;
        Ok(self)
    }

    /// 口座からの出金
    pub fn withdraw(mut self, amount: Money) -> Result<BankAccount, MoneyError> {
        self.balance = self.balance.subtract(amount)?;
        Ok(self)
    }
}
