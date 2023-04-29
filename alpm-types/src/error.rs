// SPDX-FileCopyrightText: 2023 David Runge <dvzrv@archlinux.org>
// SPDX-License-Identifier: LGPL-3.0-or-later
use thiserror::Error;

/// The Error that can occur when using types
#[derive(Debug, Error, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// An invalid build date (in seconds since the epoch)
    #[error("Invalid build date: {0}")]
    InvalidBuildDate(String),
    /// An invalid compressed file size (in bytes)
    #[error("Invalid compressed size: {0}")]
    InvalidCompressedSize(String),
    /// An invalid installed package size (in bytes)
    #[error("Invalid installed size: {0}")]
    InvalidInstalledSize(String),
    /// An invalid package name
    #[error("Invalid package name: {0}")]
    InvalidName(String),
    #[error("Invalid md5sum: {0}")]
    InvalidMd5Sum(String),
    #[error("Invalid version string: {0}")]
    InvalidVersion(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("Invalid build date: -1", Error::InvalidBuildDate(String::from("-1")))]
    #[case(
        "Invalid compressed size: -1",
        Error::InvalidCompressedSize(String::from("-1"))
    )]
    #[case(
        "Invalid installed size: -1",
        Error::InvalidInstalledSize(String::from("-1"))
    )]
    #[case("Invalid package name: -1", Error::InvalidName(String::from("-1")))]
    #[case("Invalid md5sum: -1", Error::InvalidMd5Sum(String::from("-1")))]
    #[case(
        "Invalid version string: -1",
        Error::InvalidVersion(String::from("-1"))
    )]
    fn error_format_string(#[case] error_str: &str, #[case] error: Error) {
        assert_eq!(error_str, format!("{}", error));
    }
}
