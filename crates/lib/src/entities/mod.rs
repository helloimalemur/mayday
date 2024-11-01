use sea_orm::DatabaseConnection;
use crate::user::{UserRequest, UserRequestType};

pub mod analytic;
pub mod appstate;
pub mod mayday;
pub mod message;
pub mod session;
pub mod user;

pub trait MaydayRequest {
    fn process(&self, dbcon: DatabaseConnection);
    fn create(&self, dbcon: DatabaseConnection) {
        println!("Mayday request");
    }
    fn read(&self, dbcon: DatabaseConnection) {
        println!("Mayday request");
    }
    fn update(&self, dbcon: DatabaseConnection) {
        println!("Mayday request");
    }
    fn delete(&self, dbcon: DatabaseConnection) {
        println!("Mayday request");
    }
}
