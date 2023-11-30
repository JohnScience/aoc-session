#![doc = include_str!("../README.md")]

use core::fmt;
use std::fmt::{Debug, Display};

/// The error type for this crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("No session cookie found")]
    NoSessionCookieFound,
    #[error("Rookie crate error: {0}")]
    RookieError(anyhow::Error),
}

/// The result type for this crate.
pub type Result<T> = core::result::Result<T, Error>;

/// Value of the session cookie for Advent of Code.
///
/// For example, this value can be used to get access to the puzzle input.
///
///  # Examples
///
/// ## Debug-print the session cookie value to stdout:
///
/// ```
/// use aoc_session::aoc_session;
///
/// let session_id: String = match aoc_session() {
///     Ok(session) => format!("{session:?}"),
///     Err(e) => panic!("Error: {e}"),
/// };
///
/// assert!(session_id.starts_with("session="));
/// println!("{}", session_id);
/// ```
///
/// ## Convert the session cookie value to a [`String`]:
///
/// ```
/// use aoc_session::aoc_session;
///
/// let session_id: String = match aoc_session() {
///     Ok(session) => session.to_string(),
///     Err(e) => panic!("Error: {e}"),
/// };
///
/// assert!(session_id.len() > 0);
/// assert!(!session_id.starts_with("session="));
/// println!("My session ID: {}", session_id);
/// ```
///
pub struct AocSession(String);

impl AocSession {
    #[cfg(test)]
    pub fn new(session: impl ToString) -> Self {
        let session = session.to_string();
        for symbol in session.chars() {
            if !('a'..='z').contains(&symbol) && !('0'..='9').contains(&symbol) {
                panic!("Session cookie value must be a lowercase string that represents a base-16 number");
            }
        }
        Self(session)
    }
}

impl Debug for AocSession {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "session={}", self.0)
    }
}

impl Display for AocSession {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Get the session cookie for Advent of Code. Beware that this function works for all browsers
/// supported by [`rookie`] but is slow.
///
/// # Examples
///
/// ## Debug-print the session cookie value to stdout:
///
/// ```
/// use aoc_session::aoc_session;
///
/// let session_id: String = match aoc_session() {
///     Ok(session) => format!("{session:?}"),
///     Err(e) => panic!("Error: {e}"),
/// };
///
/// assert!(session_id.starts_with("session="));
/// println!("{}", session_id);
/// ```
///
/// ## Convert the session cookie value to a [`String`]:
///
/// ```
/// use aoc_session::aoc_session;
///
/// let session_id: String = match aoc_session() {
///     Ok(session) => session.to_string(),
///     Err(e) => panic!("Error: {e}"),
/// };
///
/// assert!(session_id.len() > 0);
/// println!("My session ID: {}", session_id);
/// ```
///
pub fn aoc_session() -> Result<AocSession> {
    let domains = Some(vec!["adventofcode.com"]); // set to None to get all
    let cookies: Vec<_> = rookie::load(domains).map_err(Error::RookieError)?;
    let session = cookies
        .into_iter()
        .find(|c| c.name == "session")
        .ok_or(Error::NoSessionCookieFound)?;
    Ok(AocSession(session.value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_session_cookie() {
        let session = match aoc_session() {
            Ok(session) => session,
            Err(e) => panic!(
                "Error: {e}.\nIf you haven't logged in to Advent of Code yet, please do so now."
            ),
        };
        // session=25a16c7465645f5f286128b604b18e3d5a906611b3eac6740672d5e471a7ab0d3af049fb7363eadb2e07edfe51b600927ddd29b2311ea418ce366e8b9cf98dcc
        println!("{:?}", session);
    }

    #[test]
    fn check_debug_format() {
        let session = AocSession::new("25a16c7465645f5f286128b604b18e3d5a906611b3eac6740672d5e471a7ab0d3af049fb7363eadb2e07edfe51b600927ddd29b2311ea418ce366e8b9cf98dcc");
        assert_eq!(
            format!("{:?}", session),
            "session=25a16c7465645f5f286128b604b18e3d5a906611b3eac6740672d5e471a7ab0d3af049fb7363eadb2e07edfe51b600927ddd29b2311ea418ce366e8b9cf98dcc"
        );
    }

    #[test]
    fn check_to_string() {
        let session = AocSession::new("25a16c7465645f5f286128b604b18e3d5a906611b3eac6740672d5e471a7ab0d3af049fb7363eadb2e07edfe51b600927ddd29b2311ea418ce366e8b9cf98dcc");
        assert_eq!(
            session.to_string(),
            "25a16c7465645f5f286128b604b18e3d5a906611b3eac6740672d5e471a7ab0d3af049fb7363eadb2e07edfe51b600927ddd29b2311ea418ce366e8b9cf98dcc"
        );
    }
}
