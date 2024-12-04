use near_sdk::json_types::U128;
use rust_decimal::{Decimal, prelude::ToPrimitive};

pub type UsdtBalance = Decimal;

pub type TaskId = String;

const USDT_DIGITS: u128 = 1_000_000;


pub trait UsdtBalanceExt {
    fn from_usdt(amount: u128) -> Decimal;
    fn to_usdt(amount: Decimal) -> U128;
}

impl UsdtBalanceExt for Decimal {
    // converts to decimal from USDT fakes money
    fn from_usdt(usdt_amount: u128) -> Decimal {
        return Decimal::from(usdt_amount / USDT_DIGITS);
    }

    // converts to USDT fakes money with 6 digits for decimals
    fn to_usdt(decimal_amount: Decimal) -> U128 {
        let validated_decimal_amount = decimal_amount
            .checked_mul(Decimal::from(USDT_DIGITS))
            .unwrap_or_else(|| panic!("Decimal conversion overflow"));
        
        let amount_u128: u128 = validated_decimal_amount.round().to_u128().unwrap();

        return U128(amount_u128);
    }
}
