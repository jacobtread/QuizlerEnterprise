//! Utility types used to maintain data integrity

use sea_orm::{sea_query::Nullable, ActiveValue, DeriveValueType, IntoActiveValue};
use serde::Serialize;
use std::{borrow::Cow, fmt::Display, str::FromStr};
use validator::{validate_email, ValidationErrors};

/// Wrapper of a string that represents an email address, the value
/// is validated and converted to lowercase before its allowed to be created.
///
/// Thus prevent the need for extra validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, DeriveValueType)]
pub struct EmailAddress(String);

impl Nullable for EmailAddress {
    fn null() -> sea_orm::prelude::Value {
        sea_orm::prelude::Value::String(None)
    }
}

impl EmailAddress {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    // Provides a cloned copy of the underlying string
    pub fn clone_inner(&self) -> String {
        self.0.clone()
    }
}

impl From<String> for EmailAddress {
    fn from(value: String) -> Self {
        value.parse().unwrap()
    }
}

impl FromStr for EmailAddress {
    type Err = ValidationErrors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let email = s.to_lowercase();
        if validate_email(&email) {
            Ok(Self(email))
        } else {
            let mut errors = ValidationErrors::new();
            let mut err = validator::ValidationError::new("email");
            err.message = Some(Cow::from("Invalid email address provided"));
            err.add_param(Cow::from("value"), &email);
            errors.add("email", err);

            Err(errors)
        }
    }
}

impl Display for EmailAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for EmailAddress {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl IntoActiveValue<String> for EmailAddress {
    fn into_active_value(self) -> sea_orm::ActiveValue<String> {
        sea_orm::ActiveValue::Set(self.0)
    }
}
