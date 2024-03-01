
use crate::chainofresponsibility::*;

struct Manager {
    next: Option<Box<dyn ReportHandler>>,
}

impl Manager {
    fn chain(next: Box<dyn ReportHandler>) -> Box<Manager> {
        Box::new(Manager { next: Some(next) })
    }
    #[allow(dead_code)] // Just for completeness
    fn last() -> Box<Manager> {
        Box::new(Manager { next : None})
    }
}

impl ReportHandler for Manager {
    fn handle(&self, report: &Report) -> Status {
        if !report.cover {
            Status::Denied("No cover sheets on TPS reports".to_string())
        } else {
            match &self.next {
                Some(next) => next.handle(report),
                None => Status::Approved,
            }
        }
    }
}

struct Director {
    next: Option<Box<dyn ReportHandler>>,
}

impl Director {
    fn chain(next: Box<dyn ReportHandler>) -> Box<Director> {
        Box::new(Director { next: Some(next) })
    }
    #[allow(dead_code)] // Just for completeness
    fn last() -> Box<Director> {
        Box::new(Director { next : None})
    }
}

impl ReportHandler for Director {
    fn handle(&self, report: &Report) -> Status {
        if report.content.len() < 7 {
            Status::Denied("I need to see more content".to_string())
        } else {
            match &self.next {
                Some(next) => next.handle(report),
                None => Status::Approved,
            }
        }
    }
}

struct VP {
    next: Option<Box<dyn ReportHandler>>,
}

impl VP {
    #[allow(dead_code)] // Just for completeness
    fn chain(next: Box<dyn ReportHandler>) -> Box<VP> {
        Box::new(VP { next: Some(next) })
    }
    fn last() -> Box<VP> {
        Box::new(VP { next : None})
    }
}

impl ReportHandler for VP {
    fn handle(&self, report: &Report) -> Status {
        if report.content.len() > 15 {
            Status::Denied("This should be more concise".to_string())
        } else {
            match &self.next {
                Some(next) => next.handle(report),
                None => Status::Approved,
            }
        }
    }
}

#[test]
fn chain_of_responsibility() {
    let manager = Manager::chain(Director::chain(VP::last()));

    let mut tps_report = Report::new("Alice", "This is a report", false);

    assert_eq!(manager.handle(&tps_report), Status::Denied("No cover sheets on TPS reports".to_string()));

    tps_report.cover = true;

    assert_eq!(manager.handle(&tps_report), Status::Denied("This should be more concise".to_string()));

    tps_report.content = "Report".to_string();

    assert_eq!(manager.handle(&tps_report), Status::Denied("I need to see more content".to_string()));

    tps_report.content = "A report".to_string();

    assert_eq!(manager.handle(&tps_report), Status::Approved);

    // By the way... I'm gonna need you to come in on Saturday.
}
