use bank_model::bank::context::context::TransferContext;
use bank_model::bank::BankAccount;
use bank_model::bank::BankAccountId;
use bank_model::bank::CurrencyCode;
use bank_model::bank::Money;
use bank_model::bank::UserAccountId;

use rust_decimal::Decimal;

fn main() {
    let id = BankAccountId::new(1);
    let user_account_id = UserAccountId::new(10);
    let amount = Decimal::new(100, 0);
    let currency = CurrencyCode::JPY;
    let balance = Money::new(amount, currency);

    let bank_account = BankAccount::new(id, user_account_id, balance);
    println!("{:?}", bank_account);

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
