pub mod roles {
    use crate::bank_account::BankAccount;
    use crate::money::{Money, MoneyError};

    /// 送金先のロール
    pub trait ReceiveRole {
        fn on_receive(self, money: Money, from: BankAccount) -> Result<Self, MoneyError>
        where
            Self: Sized;
    }

    /// 送金元のロール
    pub trait SenderRole<T> {
        fn send(self, money: Money, to: T) -> Result<(Self, T), MoneyError>
        where
            Self: Sized;
    }
}

mod roles_impl {
    use crate::bank_account::roles::roles::{ReceiveRole, SenderRole};
    use crate::bank_account::BankAccount;
    use crate::money::{Money, MoneyError};

    /// 送金先のロール
    impl ReceiveRole for BankAccount {
        fn on_receive(self, money: Money, _from: BankAccount) -> Result<Self, MoneyError>
        where
            Self: Sized,
        {
            let new_state = self.deposit(money)?;
            Ok(new_state)
        }
    }

    /// 送金元のロール
    impl<T: ReceiveRole> SenderRole<T> for BankAccount {
        fn send(self, money: Money, to: T) -> Result<(Self, T), MoneyError>
        where
            Self: Sized,
        {
            let new_from = self.withdraw(money.clone())?;
            let new_to = to.on_receive(money, new_from.clone())?;
            Ok((new_from, new_to))
        }
    }
}
