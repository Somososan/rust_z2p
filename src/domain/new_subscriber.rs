use crate::routes::FormData;

use super::SubscriberEmail;
use super::SubscriberName;

#[derive(Debug)]
pub struct NewSubscriber {
    pub name: SubscriberName,
    pub email: SubscriberEmail,
}

impl NewSubscriber {
    pub fn parse(formdata: FormData) -> Result<Self, String> {
        let name = SubscriberName::parse(formdata.name)?;
        let email = SubscriberEmail::parse(formdata.email)?;

        Ok(NewSubscriber { name, email })
    }
}
