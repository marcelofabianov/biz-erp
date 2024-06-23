use crate::event::AccountCreatedEvent;

pub enum Register {
    AccountCreated(AccountCreatedEvent),
}
