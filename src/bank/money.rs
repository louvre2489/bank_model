use crate::bank::CurrencyCode;
use crate::bank::MoneyError;
use rust_decimal::Decimal;

use std::ops::Add;

#[derive(Debug, Clone, PartialEq)]
pub struct Money {
    amount: Decimal,
    currency: CurrencyCode,
}

impl Money {
    pub fn new(amount: Decimal, currency: CurrencyCode) -> Self {
        Self { amount, currency }
    }

    pub fn from(pair: (u32, CurrencyCode)) -> Self {
        let (amount, currency) = pair;

        Self {
            amount: Decimal::new(amount.into(), 0),
            currency,
        }
    }

    pub fn yens_u32(yen: i64) -> Self {
        let amount = Decimal::new(yen, 0);
        Self {
            amount,
            currency: CurrencyCode::JPY,
        }
    }

    pub fn add(&self, amount: Self) -> Result<Self, MoneyError> {
        Ok(Self {
            amount: self.amount + amount.amount,
            currency: amount.currency,
        })
    }

    pub fn subtract(&self, amount: Self) -> Result<Self, MoneyError> {
        Ok(Self {
            amount: self.amount - amount.amount,
            currency: amount.currency,
        })
    }
}

impl std::ops::Add for Money {
    type Output = Result<Money, MoneyError>;

    fn add(mut self, rhs: Self) -> Self::Output {
        if self.currency != rhs.currency {
            Err(MoneyError::NotSameCurrencyError)
        } else {
            self.amount += rhs.amount;
            Ok(self)
        }
    }
}

impl std::ops::Sub for Money {
    type Output = Result<Money, MoneyError>;

    fn sub(mut self, rhs: Self) -> Self::Output {
        if self.currency != rhs.currency {
            Err(MoneyError::NotSameCurrencyError)
        } else {
            self.amount -= rhs.amount;
            Ok(self)
        }
    }
}

#[test]
fn test_add() {
    let m1 = Money::from((1u32, CurrencyCode::USD));
    let m2 = Money::from((2u32, CurrencyCode::USD));

    fn add<T: Add<Output = Result<T, MoneyError>>>(v1: T, v2: T) -> Result<T, MoneyError> {
        v1 + v2
    }

    let m3 = add(m1, m2);
    println!("{:?}", m3);
}
