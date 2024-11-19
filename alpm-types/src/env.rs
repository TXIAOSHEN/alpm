use std::{
    fmt::{Display, Formatter},
    str::FromStr,
    string::ToString,
};

use crate::error::Error;
use crate::Architecture;
use crate::Name;
use crate::Version;

/// An option string
///
/// The option string is identified by its name and whether it is on (not prefixed with "!") or off
/// (prefixed with "!").
///
/// This type is used in the context of `makepkg` options, build environment options ([`BuildEnv`]),
/// and package options ([`PackageOption`]).
///
/// ## Examples
/// ```
/// use alpm_types::MakePkgOption;
///
/// let option = MakePkgOption::new("foo").unwrap();
/// assert_eq!(option.on(), true);
/// assert_eq!(option.name(), "foo");
///
/// let not_option = MakePkgOption::new("!foo").unwrap();
/// assert_eq!(not_option.on(), false);
/// assert_eq!(not_option.name(), "foo");
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MakePkgOption {
    name: String,
    on: bool,
}

impl MakePkgOption {
    /// Create a new MakePkgOption in a Result
    pub fn new(option: &str) -> Result<Self, Error> {
        let (name, on) = if let Some(name) = option.strip_prefix('!') {
            (name.to_owned(), false)
        } else {
            (option.to_owned(), true)
        };
        if let Some(c) = name
            .chars()
            .find(|c| !(c.is_alphanumeric() || ['-', '.', '_'].contains(c)))
        {
            return Err(Error::ValueContainsInvalidChars { invalid_char: c });
        }
        Ok(MakePkgOption { name, on })
    }

    /// Get the name of the MakePkgOption
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get whether the MakePkgOption is on
    pub fn on(&self) -> bool {
        self.on
    }
}

impl FromStr for MakePkgOption {
    type Err = Error;
    /// Create an Option from a string
    fn from_str(input: &str) -> Result<MakePkgOption, Self::Err> {
        MakePkgOption::new(input)
    }
}

impl Display for MakePkgOption {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        write!(fmt, "{}{}", if self.on { "" } else { "!" }, self.name)
    }
}

/// An option string used in a build environment
///
/// The option string is identified by its name and whether it is on (not prefixed with "!") or off
/// (prefixed with "!"). This type is an alias for [`MakePkgOption`].
///
/// ## Examples
/// ```
/// use alpm_types::BuildEnv;
///
/// let option = BuildEnv::new("foo").unwrap();
/// assert_eq!(option.on(), true);
/// assert_eq!(option.name(), "foo");
///
/// let not_option = BuildEnv::new("!foo").unwrap();
/// assert_eq!(not_option.on(), false);
/// assert_eq!(not_option.name(), "foo");
/// ```
pub type BuildEnv = MakePkgOption;

/// An option string used in packaging
///
/// The option string is identified by its name and whether it is on (not prefixed with "!") or off
/// (prefixed with "!"). This type is an alias for [`MakePkgOption`].
///
/// ## Examples
/// ```
/// use alpm_types::PackageOption;
///
/// let option = PackageOption::new("foo").unwrap();
/// assert_eq!(option.on(), true);
/// assert_eq!(option.name(), "foo");
///
/// let not_option = PackageOption::new("!foo").unwrap();
/// assert_eq!(not_option.on(), false);
/// assert_eq!(not_option.name(), "foo");
/// ```
pub type PackageOption = MakePkgOption;

/// Information on an installed package in an environment
///
/// Tracks a `Name`, `Version` (which is guaranteed to have a `Pkgrel`) and `Architecture` of a
/// package in an environment.
///
/// ## Examples
/// ```
/// use alpm_types::InstalledPackage;
///
/// assert!(InstalledPackage::new("foo-bar-1:1.0.0-1-any").is_ok());
/// assert!(InstalledPackage::new("foo-bar-1:1.0.0-1").is_err());
/// assert!(InstalledPackage::new("foo-bar-1:1.0.0-any").is_err());
/// assert!(InstalledPackage::new("1:1.0.0-1-any").is_err());
/// ```
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct InstalledPackage {
    name: Name,
    version: Version,
    architecture: Architecture,
}

impl InstalledPackage {
    /// Create new Installed and return it in a Result
    pub fn new(installed: &str) -> Result<Self, Error> {
        const DELIMITER: char = '-';
        let mut parts = installed.rsplitn(4, DELIMITER);

        let architecture = parts.next().ok_or(Error::MissingComponent {
            component: "architecture",
        })?;
        let architecture = architecture.parse()?;
        let version = {
            let Some(pkgrel) = parts.next() else {
                return Err(Error::MissingComponent {
                    component: "pkgrel",
                })?;
            };
            let Some(epoch_pkgver) = parts.next() else {
                return Err(Error::MissingComponent {
                    component: "epoch_pkgver",
                })?;
            };
            epoch_pkgver.to_string() + "-" + pkgrel
        };
        let name = parts
            .next()
            .ok_or(Error::MissingComponent { component: "name" })?
            .to_string();

        Ok(InstalledPackage {
            name: Name::new(name)?,
            version: Version::with_pkgrel(version.as_str())?,
            architecture,
        })
    }
}

impl FromStr for InstalledPackage {
    type Err = Error;
    /// Create an Installed from a string
    fn from_str(input: &str) -> Result<InstalledPackage, Self::Err> {
        InstalledPackage::new(input)
    }
}

impl Display for InstalledPackage {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        write!(fmt, "{}-{}-{}", self.name, self.version, self.architecture)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("something", Ok(MakePkgOption{name: "something".to_string(), on: true}))]
    #[case("1cool.build-option", Ok(MakePkgOption{name: "1cool.build-option".to_string(), on: true}))]
    #[case("üñıçøĐë", Ok(MakePkgOption{name: "üñıçøĐë".to_string(), on: true}))]
    #[case("!üñıçøĐë", Ok(MakePkgOption{name: "üñıçøĐë".to_string(), on: false}))]
    #[case("!something", Ok(MakePkgOption{name: "something".to_string(), on: false}))]
    #[case("!!something", Err(Error::ValueContainsInvalidChars { invalid_char: '!'}))]
    #[case("foo\\", Err(Error::ValueContainsInvalidChars { invalid_char: '\\'}))]
    fn makepkgoption(#[case] from_str: &str, #[case] result: Result<MakePkgOption, Error>) {
        assert_eq!(MakePkgOption::from_str(from_str), result);
    }

    #[rstest]
    #[case(
        "foo-bar-1:1.0.0-1-any",
        Ok(InstalledPackage{
            name: Name::new("foo-bar".to_string()).unwrap(),
            version: Version::new("1:1.0.0-1").unwrap(),
            architecture: Architecture::Any,
        }),
    )]
    #[case("foo-bar-1:1.0.0-1", Err(strum::ParseError::VariantNotFound.into()))]
    #[case("foo-bar-1:1.0.0-any", Err(Error::InvalidInteger{ kind: std::num::IntErrorKind::InvalidDigit}))]
    #[case("1:1.0.0-1-any", Err(Error::MissingComponent { component: "name" }))]
    fn installed_new(#[case] from_str: &str, #[case] result: Result<InstalledPackage, Error>) {
        assert_eq!(InstalledPackage::new(from_str), result);
    }
}
