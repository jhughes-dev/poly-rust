#[derive(Debug,PartialEq)]
pub enum Status {
    Approved,
    Denied(String /*Denial Reason*/)
}

pub trait ReportHandler {
    fn handle(&self, report: &Report) -> Status;
}

pub struct Report {
    pub owner: String,
    pub content: String,
    pub cover: bool,
}

impl Report {
    pub fn new(owner: &str, content: &str, cover: bool) -> Report {
        Report {
            owner: owner.to_string(),
            content: content.to_string(),
            cover,
        }
    }
}
