#![deny(missing_docs)]
#![doc = include_str!("../README.md")]
#![cfg_attr(test, deny(warnings))]

extern crate failure;
#[cfg(unix)]
extern crate privdrop;
#[macro_use]
extern crate structopt;

use failure::Error;

use std::ffi::OsString;
#[cfg(feature = "chroot")]
use std::path::PathBuf;

#[cfg(unix)]
mod supported {
  use super::*;

  /// Drop permissions of a CLI using structopt.
  #[derive(StructOpt, Debug)]
  pub struct Permission {
    /// Change the process user
    #[structopt(short = "u", long = "user", parse(from_os_str))]
    user: Option<OsString>,
    /// Change the process group
    #[structopt(short = "g", long = "group", parse(from_os_str))]
    group: Option<OsString>,
    /// Change the process root directory
    #[cfg(feature = "chroot")]
    #[structopt(long = "chroot", parse(from_os_str))]
    chroot: Option<PathBuf>,
  }

  impl Permission {
    /// Drop privileges.
    pub fn drop(self) -> Result<(), Error> {
      let mut drop = privdrop::PrivDrop::default();

      if let Some(user) = self.user {
        drop = drop.user(&user);
      }

      if let Some(group) = self.group {
        drop = drop.group(&group);
      }

      #[cfg(feature = "chroot")]
      {
        if let Some(chroot) = self.chroot {
          drop = drop.chroot(&chroot);
        }
      }

      drop.apply()?;
      Ok(())
    }

    /// Get the user.
    pub fn user(&self) -> &Option<OsString> {
      &self.user
    }

    /// Get the group.
    pub fn group(&self) -> &Option<OsString> {
      &self.group
    }

    /// Get the chroot directory.
    #[cfg(feature = "chroot")]
    pub fn chroot(&self) -> &Option<PathBuf> {
      &self.chroot
    }
  }
}

#[cfg(not(unix))]
mod stub {
  use super::*;

  /// Drop permissions of a CLI using structopt.
  #[derive(StructOpt, Debug)]
  pub struct Permission {}

  impl Permission {
    /// Drop privileges.
    pub fn drop(self) -> Result<(), Error> {
      Ok(())
    }

    /// Get the user.
    pub fn user(&self) -> &Option<OsString> {
      &None
    }

    /// Get the group.
    pub fn group(&self) -> &Option<OsString> {
      &None
    }

    /// Get the chroot directory.
    #[cfg(feature = "chroot")]
    pub fn chroot(&self) -> &Option<PathBuf> {
      &None
    }
  }
}

#[cfg(unix)]
pub use supported::*;

#[cfg(not(unix))]
pub use stub::*;
