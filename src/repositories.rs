use strum_macros;

pub mod user;
pub mod error;

#[derive(strum_macros::EnumMessage, Debug)]
#[allow(dead_code)]
pub enum ErrorMessage {
    #[strum(message = "This nickname is already in use")]
    ExistingNickame,
    #[strum(message = "This email is already in use")]
    ExistingEmail,
}
