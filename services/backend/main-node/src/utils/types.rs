//! Utility types used to maintain data integrity

use sea_orm::{sea_query::Nullable, DeriveValueType, IntoActiveValue};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt::Display, str::FromStr};
use validator::{validate_email, validate_length, HasLen, ValidationError, ValidationErrors};

/// Defines an additional transformation and validation step that
/// happens post deserialization but pre validation, allowing fields
/// to be trimmed and pre-checked.
pub trait TransformValidate: Sized {
    /// Transform and validate the value
    fn transform_validate(&mut self) -> Result<(), ValidationErrors> {
        Ok(())
    }
}

/// Wrapper of a string that represents an email address, the value
/// is validated and converted to lowercase before its allowed to be created.
///
/// Thus prevent the need for extra validation.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, DeriveValueType)]
pub struct EmailAddress(String);

impl HasLen for EmailAddress {
    fn length(&self) -> u64 {
        self.0.length()
    }
}

impl TransformValidate for EmailAddress {
    fn transform_validate(&mut self) -> Result<(), ValidationErrors> {
        *self = self.0.parse()?;
        Ok(())
    }
}

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

    pub fn into_inner(self) -> String {
        self.0
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
        let email = s.trim().to_lowercase();
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

/// Wrapper of a string that represents an email address, the value
/// is validated and converted to lowercase before its allowed to be created.
///
/// Thus prevent the need for extra validation.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, DeriveValueType)]
pub struct Username(String);

impl HasLen for Username {
    fn length(&self) -> u64 {
        self.0.length()
    }
}

impl TransformValidate for Username {
    fn transform_validate(&mut self) -> Result<(), ValidationErrors> {
        *self = self.0.parse()?;
        Ok(())
    }
}

impl Nullable for Username {
    fn null() -> sea_orm::prelude::Value {
        sea_orm::prelude::Value::String(None)
    }
}

impl Username {
    const MIN_LENGTH: u64 = 4;
    const MAX_LENGTH: u64 = 100;

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    // Provides a cloned copy of the underlying string
    pub fn clone_inner(&self) -> String {
        self.0.clone()
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl From<String> for Username {
    fn from(value: String) -> Self {
        value.parse().unwrap()
    }
}

impl FromStr for Username {
    type Err = ValidationErrors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let username = s.trim().to_lowercase();
        if validate_length(
            &username,
            Some(Self::MIN_LENGTH),
            Some(Self::MAX_LENGTH),
            None,
        ) {
            Ok(Self(username))
        } else {
            let mut errors = ValidationErrors::new();

            let mut err = ValidationError::new("length");
            err.message = Some(Cow::from(
                "Username must be within 4 to 100 characters long",
            ));
            err.add_param(Cow::from("min"), &4u64);
            err.add_param(Cow::from("max"), &100u64);
            err.add_param(Cow::from("value"), &username);
            errors.add("username", err);

            Err(errors)
        }
    }
}

impl Display for Username {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl IntoActiveValue<String> for Username {
    fn into_active_value(self) -> sea_orm::ActiveValue<String> {
        sea_orm::ActiveValue::Set(self.0)
    }
}
