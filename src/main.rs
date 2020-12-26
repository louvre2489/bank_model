use bank_model::bank::BankAccount;
use bank_model::bank::BankAccountId;
use bank_model::bank::Money;
use bank_model::bank::UserAccountId;

fn main() {
    let id = BankAccountId::new(1);
    let user_account_id = UserAccountId::new(10);
    let balance = Money::new(100);
    let bank_account = BankAccount::new(id, user_account_id, balance);
    println!("{:?}", bank_account);
}
