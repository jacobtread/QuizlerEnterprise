//! Utility types used to maintain data integrity

use garde::Validate;
use sea_orm::{sea_query::Nullable, DeriveValueType, IntoActiveValue};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

/// Wrapper of a string that represents an email address, the value
/// is validated and converted to lowercase before its allowed to be created.
///
/// Thus prevent the need for extra validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, DeriveValueType, garde::Validate)]
#[garde(transparent)]
pub struct EmailAddress(#[garde(email)] String);

/// Extended deserializer that trims and converts the value to lowercase
impl<'de> Deserialize<'de> for EmailAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: &'de str = <&'de str>::deserialize(deserializer)?;
        Ok(Self(EmailAddress::prepare(value)))
    }
}

impl Nullable for EmailAddress {
    fn null() -> sea_orm::prelude::Value {
        sea_orm::prelude::Value::String(None)
    }
}

impl EmailAddress {
    /// Prepares the provided value for being converted into an email
    /// by trimming whitespace and converting to lowercase
    pub fn prepare(value: &str) -> String {
        value.trim().to_lowercase()
    }

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

impl FromStr for EmailAddress {
    type Err = garde::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let email = EmailAddress(EmailAddress::prepare(s));
        email.validate(&())?;
        Ok(email)
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, DeriveValueType, garde::Validate)]
#[garde(transparent)]
pub struct Username(#[garde(alphanumeric, length(min = 4, max = 100))] String);

impl<'de> Deserialize<'de> for Username {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: &'de str = <&'de str>::deserialize(deserializer)?;
        Ok(Self(Username::prepare(value)))
    }
}

impl Nullable for Username {
    fn null() -> sea_orm::prelude::Value {
        sea_orm::prelude::Value::String(None)
    }
}

impl Username {
    /// Prepares the provided value for being converted into a username
    /// by trimming whitespace and converting to lowercase
    fn prepare(value: &str) -> String {
        value.trim().to_lowercase()
    }

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
    type Err = garde::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let username = Username(Username::prepare(s));
        username.validate(&())?;
        Ok(username)
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

#[derive(Clone, PartialEq, Eq, Serialize, DeriveValueType, garde::Validate)]
#[garde(transparent)]
pub struct Password(#[garde(ascii, length(min = 4, max = 100))] String);

impl Nullable for Password {
    fn null() -> sea_orm::prelude::Value {
        sea_orm::prelude::Value::String(None)
    }
}

impl Password {
    /// Prepares the provided value for being converted into a password
    /// by trimming whitespace
    pub fn prepare(value: &str) -> String {
        value.trim().to_string()
    }

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

impl FromStr for Password {
    type Err = garde::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let password = Password(Password::prepare(s));
        password.validate(&())?;
        Ok(password)
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Redacted(Password)")
    }
}

impl Debug for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Redacted(Password)")
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl IntoActiveValue<String> for Password {
    fn into_active_value(self) -> sea_orm::ActiveValue<String> {
        sea_orm::ActiveValue::Set(self.0)
    }
}

impl<'de> Deserialize<'de> for Password {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: &'de str = <&'de str>::deserialize(deserializer)?;
        Ok(Self(Password::prepare(value)))
    }
}
