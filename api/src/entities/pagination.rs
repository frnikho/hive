#[derive(Clone, Debug)]
pub struct Pagination {
    pub page: u16,
    pub limit: u16,
    pub bypass: bool,
}

impl Pagination {
    pub fn exec() -> String {
        "".to_string()
    }

    pub fn bypass() -> Self {
        Self {
            page: 0,
            limit: 100,
            bypass: true,
        }
    }
}