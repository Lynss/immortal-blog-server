use std::collections::HashMap;
pub struct Identity {
    pub permissions: HashMap<String, i32>,
    pub roles: Vec<String>,
}
