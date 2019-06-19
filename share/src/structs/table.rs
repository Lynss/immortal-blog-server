#[derive(Deserialize, Serialize, Debug)]
pub struct OrderInfo {
    pub field: String,
    pub order: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Pagination {
    pub per_page: i64,
    pub page: i64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TableRequest<T> {
    pub data: Option<T>,
    pub pagination: Pagination,
    pub orders: Option<Vec<OrderInfo>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TableResponse<T> {
    pub datasource: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}
