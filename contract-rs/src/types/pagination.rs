use near_sdk::{ require, serde::{ Deserialize, Serialize }};

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Pagination {
    page: u8,
    size: u8,
}

impl Default for Pagination {
    fn default() -> Self {
        Self { page: 1, size: 10 }
    }
}

impl Pagination {
    pub fn new(page: u8, size: u8) -> Self {
        return Self {
            page,
            size,
        };
    }

    pub fn assert_valid(&self) {
        require!(self.size <= 50, "A single page can't contain more than 50 elements")
    }

    pub fn take(&self) -> usize {
        self.size.into()
    }

    pub fn skip(&self) -> usize {
        (self.size * (self.page - 1)).into()
    }
}