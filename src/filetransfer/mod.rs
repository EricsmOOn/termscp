//! ## FileTransfer
//!
//! `filetransfer` is the module which provides the file transfer protocols and remotefs builders

mod builder;
pub mod params;

// -- export types
pub use builder::Builder;
pub use params::{FileTransferParams, ProtocolParams};

/// This enum defines the different transfer protocol available in termscp

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum FileTransferProtocol {
    Sftp,
    Scp,
    Ftp(bool), // Bool is for secure (true => ftps)
    AwsS3,
}

// Traits

impl std::string::ToString for FileTransferProtocol {
    fn to_string(&self) -> String {
        String::from(match self {
            FileTransferProtocol::Ftp(secure) => match secure {
                true => "FTPS",
                false => "FTP",
            },
            FileTransferProtocol::Scp => "SCP",
            FileTransferProtocol::Sftp => "SFTP",
            FileTransferProtocol::AwsS3 => "S3",
        })
    }
}

impl std::str::FromStr for FileTransferProtocol {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "FTP" => Ok(FileTransferProtocol::Ftp(false)),
            "FTPS" => Ok(FileTransferProtocol::Ftp(true)),
            "SCP" => Ok(FileTransferProtocol::Scp),
            "SFTP" => Ok(FileTransferProtocol::Sftp),
            "S3" => Ok(FileTransferProtocol::AwsS3),
            _ => Err(s.to_string()),
        }
    }
}

// Tests

#[cfg(test)]
mod tests {

    use super::*;

    use pretty_assertions::assert_eq;
    use std::str::FromStr;
    use std::string::ToString;

    #[test]
    fn test_filetransfer_mod_protocol() {
        assert_eq!(
            FileTransferProtocol::Ftp(true),
            FileTransferProtocol::Ftp(true)
        );
        assert_eq!(
            FileTransferProtocol::Ftp(false),
            FileTransferProtocol::Ftp(false)
        );
        // From str
        assert_eq!(
            FileTransferProtocol::from_str("FTPS").ok().unwrap(),
            FileTransferProtocol::Ftp(true)
        );
        assert_eq!(
            FileTransferProtocol::from_str("ftps").ok().unwrap(),
            FileTransferProtocol::Ftp(true)
        );
        assert_eq!(
            FileTransferProtocol::from_str("FTP").ok().unwrap(),
            FileTransferProtocol::Ftp(false)
        );
        assert_eq!(
            FileTransferProtocol::from_str("ftp").ok().unwrap(),
            FileTransferProtocol::Ftp(false)
        );
        assert_eq!(
            FileTransferProtocol::from_str("SFTP").ok().unwrap(),
            FileTransferProtocol::Sftp
        );
        assert_eq!(
            FileTransferProtocol::from_str("sftp").ok().unwrap(),
            FileTransferProtocol::Sftp
        );
        assert_eq!(
            FileTransferProtocol::from_str("SCP").ok().unwrap(),
            FileTransferProtocol::Scp
        );
        assert_eq!(
            FileTransferProtocol::from_str("scp").ok().unwrap(),
            FileTransferProtocol::Scp
        );
        assert_eq!(
            FileTransferProtocol::from_str("S3").ok().unwrap(),
            FileTransferProtocol::AwsS3
        );
        assert_eq!(
            FileTransferProtocol::from_str("s3").ok().unwrap(),
            FileTransferProtocol::AwsS3
        );
        // Error
        assert!(FileTransferProtocol::from_str("dummy").is_err());
        // To String
        assert_eq!(
            FileTransferProtocol::Ftp(true).to_string(),
            String::from("FTPS")
        );
        assert_eq!(
            FileTransferProtocol::Ftp(false).to_string(),
            String::from("FTP")
        );
        assert_eq!(FileTransferProtocol::Scp.to_string(), String::from("SCP"));
        assert_eq!(FileTransferProtocol::Sftp.to_string(), String::from("SFTP"));
        assert_eq!(FileTransferProtocol::AwsS3.to_string(), String::from("S3"));
    }
}
