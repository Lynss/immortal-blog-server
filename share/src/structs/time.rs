use chrono::NaiveDateTime;

#[derive(Deserialize, Serialize, Debug)]
pub struct TimeRange {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}
