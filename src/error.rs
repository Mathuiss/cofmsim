#[derive(Debug)]
#[allow(dead_code)]
pub struct SimError {
    msg: String,
}

impl From<std::io::Error> for SimError {
    fn from(value: std::io::Error) -> Self {
        Self {
            msg: format!("{:?}", value),
        }
    }
}

impl From<toml::de::Error> for SimError {
    fn from(value: toml::de::Error) -> Self {
        Self {
            msg: format!("{:?}", value),
        }
    }
}

impl From<csv::Error> for SimError {
    fn from(value: csv::Error) -> Self {
        Self {
            msg: format!("{:?}", value),
        }
    }
}
