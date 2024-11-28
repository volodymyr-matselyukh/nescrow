use near_sdk::json_types::U128;
use rust_decimal::{Decimal, prelude::ToPrimitive};

pub type UsdtBalance = U128;

pub type TaskId = String;

const USDT_DIGITS: u128 = 1_000_000;


pub trait UsdtBalanceExt {
    fn from_usdt(amount: u128) -> U128;
    fn to_usdt(amount: U128) -> U128;
    fn from_decimal(decimal: Decimal) -> UsdtBalance;
    fn to_decimal(amount: U128) -> Decimal;
}

impl UsdtBalanceExt for U128 {
    // converts to USDT fakes money with 6 digits
    fn from_usdt(amount: u128) -> U128 {
        return U128(
            amount
                .checked_mul(USDT_DIGITS)
                .unwrap_or_else(|| panic!("Usdt balance overflow")),
        );
    }

    // converts to human money
    fn to_usdt(amount: U128) -> U128 {
        return U128(amount.0 / USDT_DIGITS);
    }

    // converts to USDT fakes money with 6 digits
    fn from_decimal(decimal_amount: Decimal) -> UsdtBalance {
        let validated_decimal_amount = decimal_amount
            .checked_mul(Decimal::from(USDT_DIGITS))
            .unwrap_or_else(|| panic!("Decimal conversion overflow"));
        
        let amount_u128: u128 = validated_decimal_amount.round().to_u128().unwrap();

        U128(amount_u128)
    }

    // converts to human money
    fn to_decimal(usdt_amount: UsdtBalance) -> Decimal {
        Decimal::from(usdt_amount.0 / USDT_DIGITS)
    }
}
