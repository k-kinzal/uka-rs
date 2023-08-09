use crate::types::v3;
use crate::types::v3::HeaderName;
use std::any::type_name;

mod on_anchor_select;
mod undefined;

pub use on_anchor_select::OnAnchorSelect;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to decode header value")]
    FailedDecode(#[from] v3::HeaderValueError),
    #[error("Undefined ID")]
    UndefinedId,
    #[error("Undefined reference")]
    UndefinedReference,
    #[error("Invalid event: {0}")]
    InvalidEvent(String),
}

pub enum Event {
    OnAnchorSelect(OnAnchorSelect),
    UndefinedEvent(UndefinedEvent),
}

impl TryFrom<v3::Request> for Event {
    type Error = Error;

    fn try_from(value: v3::Request) -> Result<Self, Self::Error> {
        match value.headers.get(&HeaderName::ID) {
            Some(id) => match id.text() {
                Ok(id) if id == type_name::<OnAnchorSelect>() => {
                    Ok(Event::OnAnchorSelect(OnAnchorSelect::try_from(value)?))
                }
                Ok(id) => Ok(Event::UndefinedEvent(UndefinedEvent {
                    id,
                    reference0: value
                        .headers
                        .get(&HeaderName::REFERENCE0)
                        .map(|v| v.text())
                        .transpose()?,
                    reference1: value
                        .headers
                        .get(&HeaderName::REFERENCE1)
                        .map(|v| v.text())
                        .transpose()?,
                    reference2: value
                        .headers
                        .get(&HeaderName::REFERENCE2)
                        .map(|v| v.text())
                        .transpose()?,
                    reference3: value
                        .headers
                        .get(&HeaderName::REFERENCE3)
                        .map(|v| v.text())
                        .transpose()?,
                    reference4: value
                        .headers
                        .get(&HeaderName::REFERENCE4)
                        .map(|v| v.text())
                        .transpose()?,
                    reference5: value
                        .headers
                        .get(&HeaderName::REFERENCE5)
                        .map(|v| v.text())
                        .transpose()?,
                    reference6: value
                        .headers
                        .get(&HeaderName::REFERENCE6)
                        .map(|v| v.text())
                        .transpose()?,
                    reference7: value
                        .headers
                        .get(&HeaderName::REFERENCE7)
                        .map(|v| v.text())
                        .transpose()?,
                })),
                Err(e) => Err(Error::FailedDecode(e)),
            },
            None => Err(Error::UndefinedId),
        }
    }
}
