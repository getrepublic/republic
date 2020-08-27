#[derive(Debug)]
pub struct Error(String);

impl Error {
    pub fn new(msg: impl ToString) -> Self {
        Error(msg.to_string())
    }
}
