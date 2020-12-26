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

    let new_ba1 = bank_account.deposit(Money::yens_u32(1000)).unwrap();
    println!("{:?}", new_ba1);
}
