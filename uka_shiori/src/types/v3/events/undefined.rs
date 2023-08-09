use crate::types::v3;

pub struct UndefinedEvent {
    id: String,
    reference0: Option<v3::HeaderValue>,
    reference1: Option<v3::HeaderValue>,
    reference2: Option<v3::HeaderValue>,
    reference3: Option<v3::HeaderValue>,
    reference4: Option<v3::HeaderValue>,
    reference5: Option<v3::HeaderValue>,
    reference6: Option<v3::HeaderValue>,
    reference7: Option<v3::HeaderValue>,
}

impl UndefinedEvent {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn reference0(&self) -> Option<&v3::HeaderValue> {
        self.reference0.as_ref()
    }

    pub fn reference1(&self) -> Option<&v3::HeaderValue> {
        self.reference1.as_ref()
    }

    pub fn reference2(&self) -> Option<&v3::HeaderValue> {
        self.reference2.as_ref()
    }

    pub fn reference3(&self) -> Option<&v3::HeaderValue> {
        self.reference3.as_ref()
    }

    pub fn reference4(&self) -> Option<&v3::HeaderValue> {
        self.reference4.as_ref()
    }

    pub fn reference5(&self) -> Option<&v3::HeaderValue> {
        self.reference5.as_ref()
    }

    pub fn reference6(&self) -> Option<&v3::HeaderValue> {
        self.reference6.as_ref()
    }

    pub fn reference7(&self) -> Option<&v3::HeaderValue> {
        self.reference7.as_ref()
    }
}

impl TryFrom<v3::Request> for UndefinedEvent {
    type Error = v3::events::Error;

    fn try_from(mut value: v3::Request) -> Result<Self, Self::Error> {
        match (
            value
                .headers
                .get(&v3::HeaderName::ID)
                .map(|v| v.text())
                .transpose()?,
            value.headers.remove(&v3::HeaderName::REFERENCE0),
            value.headers.remove(&v3::HeaderName::REFERENCE1),
            value.headers.remove(&v3::HeaderName::REFERENCE2),
            value.headers.remove(&v3::HeaderName::REFERENCE3),
            value.headers.remove(&v3::HeaderName::REFERENCE4),
            value.headers.remove(&v3::HeaderName::REFERENCE5),
            value.headers.remove(&v3::HeaderName::REFERENCE6),
            value.headers.remove(&v3::HeaderName::REFERENCE7),
        ) {
            (Some(id), r0, r1, r2, r3, r4, r5, r6, r7) => Ok(Self {
                id,
                reference0: r0,
                reference1: r1,
                reference2: r2,
                reference3: r3,
                reference4: r4,
                reference5: r5,
                reference6: r6,
                reference7: r7,
            }),
            _ => Err(v3::events::Error::UndefinedId),
        }
    }
}
