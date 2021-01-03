use bank_model::bank_account::context::context::TransferContext;
use bank_model::bank_account::BankAccount;
use bank_model::bank_account::BankAccountId;
use bank_model::bank_account::UserAccountId;
use bank_model::currency::CurrencyCode;
use bank_model::money::Money;

fn main() {
    let ba1 = BankAccount::new(
        BankAccountId::new(1),
        UserAccountId::new(1),
        Money::zero(CurrencyCode::JPY),
    );

    let new_ba1 = ba1.deposit(Money::yens_i32(1000)).unwrap();

    let ba2 = BankAccount::new(
        BankAccountId::new(2),
        UserAccountId::new(1),
        Money::zero(CurrencyCode::JPY),
    );

    let context: TransferContext<BankAccount, BankAccount> = TransferContext::new(new_ba1, ba2);
    let (from, to) = context.transfer(Money::yens_i32(10)).unwrap();

    println!("from = {:?}, to = {:?}", from, to);
}
