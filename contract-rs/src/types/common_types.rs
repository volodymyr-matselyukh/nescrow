use near_sdk::json_types::U128;

pub type UsdtBalance = U128;

const USDT_DIGITS: u128 = 1_000_000;

pub trait UsdtBalanceExt {
    fn from_usdt(amount: u128) -> U128;
    fn to_usdt(amount: U128) -> U128;
}

impl UsdtBalanceExt for U128 {
    fn from_usdt(amount: u128) -> U128 {
        return U128(
            amount
                .checked_mul(USDT_DIGITS)
                .unwrap_or_else(|| panic!("Usdt balance overflow")),
        );
    }

    fn to_usdt(amount: U128) -> U128 {
        return U128(amount.0 / USDT_DIGITS);
    }
}
