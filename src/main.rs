use bank_model::bank_account_id::BankAccountId;

fn main() {
    let bank_account_id = BankAccountId::new(1);
    println!("{:?}", bank_account_id);
}