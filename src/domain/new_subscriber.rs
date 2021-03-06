use super::SubscriberEmail;
use super::SubscriberName;

#[derive(Debug)]
pub struct NewSubscriber {
    pub name: SubscriberName,
    pub email: SubscriberEmail,
}
