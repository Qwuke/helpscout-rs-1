pub mod conversations;
pub mod customers;
pub mod mailboxes;
pub mod users;
pub mod person;
pub mod reports;

use chrono::{DateTime, Utc};

use self::users::UsersBuilder;
use self::reports::ReportBuilder;

pub fn users() -> UsersBuilder {
    UsersBuilder::default()
}

pub fn report(start: DateTime<Utc>, end: DateTime<Utc>) -> ReportBuilder {
    ReportBuilder::new(start, end)
}
