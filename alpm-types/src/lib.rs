#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod checksum;
pub use checksum::{
    Blake2b512Checksum,
    Checksum,
    Digest,
    Md5Checksum,
    Sha1Checksum,
    Sha224Checksum,
    Sha256Checksum,
    Sha384Checksum,
    Sha512Checksum,
    SkippableChecksum,
};

mod source;
pub use source::Source;

mod url;
pub use url::{SourceUrl, Url};

/// Public re-exports of common hash functions, for use with [`Checksum`].
pub mod digests {
    pub use blake2::Blake2b512;
    pub use digest::Digest;
    pub use md5::Md5;
    pub use sha1::Sha1;
    pub use sha2::{Sha224, Sha256, Sha384, Sha512};
}

mod date;
pub use date::{BuildDate, FromOffsetDateTime};

mod env;
pub use env::{BuildEnvironmentOption, InstalledPackage, MakepkgOption, PackageOption};

mod error;
pub use error::Error;

mod license;
pub use license::License;

mod name;
pub use name::{BuildTool, Name};

mod path;
pub use path::{
    AbsolutePath,
    Backup,
    BuildDirectory,
    Changelog,
    Install,
    RelativePath,
    StartDirectory,
};

mod openpgp;
pub use openpgp::{OpenPGPIdentifier, OpenPGPKeyId, OpenPGPv4Fingerprint, Packager};

mod pkg;
pub use pkg::{ExtraData, PackageBaseName, PackageDescription, PackageType};

mod relation;
pub use relation::{Group, OptionalDependency, PackageRelation};

mod size;
pub use size::{CompressedSize, InstalledSize};

mod system;
pub use system::Architecture;

mod version;
pub use version::{
    BuildToolVersion,
    Epoch,
    PackageRelease,
    PackageVersion,
    SchemaVersion,
    Version,
    VersionComparison,
    VersionRequirement,
};
