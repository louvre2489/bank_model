/// 送金コンテキスト
/// BankAcountには非依存
/// 送金できるT型として定義する
pub mod context {
    use crate::bank_account::roles::roles::{ReceiveRole, SenderRole};
    use crate::money::{Money, MoneyError};

    pub struct TransferContext<T: ReceiveRole, F: SenderRole<T>> {
        from: F,
        to: T,
    }

    impl<T: ReceiveRole, F: SenderRole<T>> TransferContext<T, F> {
        pub fn new(from: F, to: T) -> Self {
            Self { from, to }
        }

        pub fn transfer(self, money: Money) -> Result<(F, T), MoneyError> {
            self.from.send(money, self.to)
        }
    }
}
