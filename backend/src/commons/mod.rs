pub mod utils;

pub enum Immortal {
    Success,
    InternalError(String),
}

struct CodeMessage {
    pub code: i32,
    pub message: String,
}

impl Immortal {
    fn value(&self) -> CodeMessage {
        match *self {
            Immortal::Success => CodeMessage {
                code: 200,
                message: "Request success".into(),
            },
            Immortal::InternalError(ref err_cause) => CodeMessage {
                code: 500,
                message: format!("Internal server error caused by {}", err_cause),
            },
        }
    }
    pub fn code(&self) -> i32 {
        self.value().code
    }
    pub fn message(&self) -> String {
        self.value().message
    }
}

pub const LOG_CONFIG: &'static str = "configs/log4rs.yaml";
