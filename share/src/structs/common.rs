use diesel::sql_types::VarChar;

#[derive(Serialize, Queryable, QueryableByName, Deserialize)]
pub struct SelectOption {
    #[sql_type = "VarChar"]
    pub id: String,
    #[sql_type = "VarChar"]
    pub name: String,
}
