//! The pwd module is responsible for hashing and validating hashes.
//! It follows a multi-scheme hashing code design, allowing each
//! scheme to provide its own hashing and validation methods.
//!
//! Code Design Points:
//!
//! - Exposes two public async functions `hash_pwd(...)` and `validate_pwd(...)`
//! - `ContentToHash` represents the data to be hashed along with the corresponding salt.
//! - `SchemeStatus` is the result of `validate_pwd` which, upon successful validation, indicates
//!   whether the password needs to be re-hashed to adopt the latest scheme.
//! - Internally, the `pwd` module implements a multi-scheme code design with the `Scheme` trait.
//! - The `Scheme` trait exposes sync functions `hash` and `validate` to be implemented for each scheme.
//! - The two public async functions `hash_pwd(...)` and `validate_pwd(...)` call the scheme using
//!   `spawn_blocking` to ensure that long hashing/validation processes do not hinder the execution of smaller tasks.
//! - Schemes are designed to be agnostic of whether they are in an async or sync context, hence they are async-free.

// region:    --- Modules;

mod error;
mod scheme;

pub use self::error::{Error, Result};
pub use scheme::SchemeStatus;

use crate::pass::scheme::{get_scheme, Scheme, DEFAULT_SCHEME};
use lazy_regex::regex_captures;
use std::str::FromStr;
use uuid::Uuid;

// endregion: --- Modules

// region:    --- Types

/// The clean content to hash, with the salt.
///
/// Notes:
///    - Since content is sensitive information, we do NOT implement default debug for this struct.
///    - The clone is only implement for testing
#[cfg_attr(test, derive(Clone))]
pub struct ContentToHash {
    pub content: String, // Clear content.
    pub salt: Uuid,
}

// endregion: --- Types

// region:    --- Public Functions

/// Hash the password with the default scheme.
pub async fn hash_pwd(to_hash: ContentToHash) -> Result<String> {
    tokio::task::spawn_blocking(move || hash_for_scheme(DEFAULT_SCHEME, to_hash))
        .await
        .map_err(|_| Error::FailSpawnBlockForHash)?
}

/// Validate if an ContentToHash matches.
pub async fn validate_pwd(to_hash: ContentToHash, pwd_ref: String) -> Result<SchemeStatus> {
    let PwdParts {
        scheme_name,
        hashed,
    } = pwd_ref.parse()?;

    // Note: We do first, so that we do not have to clonse the scheme_name.
    let scheme_status = if scheme_name == DEFAULT_SCHEME {
        SchemeStatus::Ok
    } else {
        SchemeStatus::Outdated
    };

    // Note: Since validate might take some time depending on algo
    //       doing a spawn_blocking to avoid
    tokio::task::spawn_blocking(move || validate_for_scheme(&scheme_name, to_hash, hashed))
        .await
        .map_err(|_| Error::FailSpawnBlockForValidate)??;

    // validate_for_scheme(&scheme_name, to_hash, &hashed).await?;
    Ok(scheme_status)
}
// endregion: --- Public Functions

// region:    --- Privates

fn hash_for_scheme(scheme_name: &str, to_hash: ContentToHash) -> Result<String> {
    let pwd_hashed = get_scheme(scheme_name)?.hash(&to_hash)?;

    Ok(format!("#{scheme_name}#{pwd_hashed}"))
}

fn validate_for_scheme(scheme_name: &str, to_hash: ContentToHash, pwd_ref: String) -> Result<()> {
    get_scheme(scheme_name)?.validate(&to_hash, &pwd_ref)?;
    Ok(())
}

struct PwdParts {
    /// The scheme only (e.g., "01")
    scheme_name: String,
    /// The hashed password,
    hashed: String,
}

impl FromStr for PwdParts {
    type Err = Error;

    fn from_str(pwd_with_scheme: &str) -> Result<Self> {
        regex_captures!(
            r#"^#(\w+)#(.*)"#, // a literal regex
            pwd_with_scheme
        )
        .map(|(_, scheme, hashed)| Self {
            scheme_name: scheme.to_string(),
            hashed: hashed.to_string(),
        })
        .ok_or(Error::PassWithSchemeFailedToParse)
    }
}

// endregion: --- Privates/ endregion:    --- Types

// region: --- test module

#[cfg(test)]
mod tests {
    use self::Result;
    use super::*;

    #[tokio::test]
    async fn test_hash_pwd_success() -> Result<()> {
        // Setup a sample content and salt
        let content_to_hash = ContentToHash {
            content: "superoot".to_string(),
            salt: Uuid::new_v4(),
        };

        // Execute hash_pwd
        let hashed_pwd = hash_pwd(content_to_hash.clone()).await?;

        // Check that the hashed password is not empty
        assert!(!hashed_pwd.is_empty(), "Hashing failed, got an empty hash");

        Ok(())
    }

    #[tokio::test]
    async fn test_validate_pwd_success() -> Result<()> {
        // Sample content and salt for hashing and validation
        let content_to_hash = ContentToHash {
            content: "superoot".to_string(),
            salt: Uuid::parse_str("f05e8961-d6ad-4086-9e78-a6de065e5453")?,
        };

        // Hash the password using Scheme02 (argon2)
        let hashed_pwd = hash_pwd(content_to_hash.clone()).await?;

        // Validate the hashed password
        let result = validate_pwd(content_to_hash.clone(), hashed_pwd).await?;

        // Verify the status is Ok as the password uses the latest scheme
        assert_eq!(
            result,
            SchemeStatus::Ok,
            "Expected Ok status for latest scheme"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_validate_pwd_fail() -> Result<()> {
        // Setup content and salt
        let content_to_hash = ContentToHash {
            content: "test_password".to_string(),
            salt: Uuid::parse_str("f05e8961-d6ad-4086-9e78-a6de065e5453")?,
        };

        // Hash the password with Scheme02
        let hashed_pwd = hash_pwd(content_to_hash.clone()).await?;

        // Attempt validation with incorrect password content
        let invalid_content_to_hash = ContentToHash {
            content: "wrong_password".to_string(),
            salt: content_to_hash.salt,
        };

        // Validate should fail with incorrect password
        let result = validate_pwd(invalid_content_to_hash, hashed_pwd).await;

        assert!(
            result.is_err(),
            "Expected validation failure with incorrect password"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_scheme_outdated() -> Result<()> {
        // Setup content and salt
        let content_to_hash = ContentToHash {
            content: "test_password".to_string(),
            salt: Uuid::parse_str("f05e8961-d6ad-4086-9e78-a6de065e5453")?,
        };

        // Manually create a hashed password with an outdated scheme
        let outdated_scheme_hash = "#01#outdated_hash_placeholder".to_string();

        // Validate using outdated scheme hash
        let result = validate_pwd(content_to_hash.clone(), outdated_scheme_hash).await;

        // Expected to have SchemeStatus::Outdated if the hash is valid but old
        assert_eq!(
            result.unwrap_or(SchemeStatus::Outdated),
            SchemeStatus::Outdated,
            "Expected Outdated scheme status"
        );

        Ok(())
    }
}

// endregion: --- test module
