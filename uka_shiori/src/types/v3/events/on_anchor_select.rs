use crate::types::v3;
use std::any::type_name;

pub struct OnAnchorSelect {
    r0: String,
}

impl OnAnchorSelect {
    pub fn r0(&self) -> &str {
        &self.r0
    }
}

impl TryFrom<v3::Request> for OnAnchorSelect {
    type Error = v3::events::Error;

    fn try_from(value: v3::Request) -> Result<Self, Self::Error> {
        match (
            value
                .headers
                .get(&v3::HeaderName::ID)
                .map(|v| v.text())
                .transpose()?,
            value
                .headers
                .get(&v3::HeaderName::REFERENCE0)
                .map(|v| v.text())
                .transpose()?,
        ) {
            (Some(id), Some(r0)) if id == type_name::<Self>() => Ok(Self { r0 }),
            (Some(id), Some(_)) => Err(v3::events::Error::InvalidEvent(id)),
            (Some(_), None) => Err(v3::events::Error::UndefinedReference),
            (None, Some(_)) => Err(v3::events::Error::UndefinedId),
            (None, None) => Err(v3::events::Error::UndefinedId),
        }
    }
}
