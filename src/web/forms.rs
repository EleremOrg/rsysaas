use serde::Deserialize;

use crate::data::{errors::CRUDError, facades::db::Manager, models::customer::PotentialCustomer};

#[derive(Deserialize, Debug)]
pub struct PotentialCustomerForm {
    pub name: String,
    pub email: String,
    pub message: String,
}

impl PotentialCustomerForm {
    pub async fn create(&self) {
        match PotentialCustomer::create(
            "name, email, message",
            format!("'{}', '{}', '{}'", self.name, self.email, self.message).as_str(),
        )
        .await
        {
            Ok(result) => println!("succesfull result: {:?}", result),
            Err(err) => {
                println!("error executing sql: {:?}", err);
                return;
            }
        };
    }
}
