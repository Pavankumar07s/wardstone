use core::fmt;
use std::path::PathBuf;
use std::process::{ExitCode, Termination};

use clap::ValueEnum;
use wardstone_core::context::Context;
use wardstone_core::primitive::hash::*;
use wardstone_core::standard::bsi::Bsi;
use wardstone_core::standard::cnsa::Cnsa;
use wardstone_core::standard::Standard;

use crate::adapter::Asymmetric;
use crate::key::certificate::Certificate;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Guide {
  /// The BSI TR-02102 series of technical guidelines.
  Bsi,
  Cnsa,
}

impl Guide {
  fn validate_hash_function(
    &self,
    ctx: &Context,
    hash: &Hash,
  ) -> Result<&'static Hash, &'static Hash> {
    match self {
      Self::Bsi => Bsi::validate_hash(ctx, hash),
      Self::Cnsa => Cnsa::validate_hash(ctx, hash),
    }
  }

  fn validate_signature_algorithm(
    &self,
    ctx: &Context,
    algorithm: &Asymmetric,
  ) -> Result<Asymmetric, Asymmetric> {
    match self {
      Self::Bsi => match algorithm {
        Asymmetric::Ecc(instance) => match Bsi::validate_ecc(ctx, instance) {
          Ok(instance) => Ok(Asymmetric::Ecc(instance)),
          Err(instance) => Err(Asymmetric::Ecc(instance)),
        },
        Asymmetric::Ifc(instance) => match Bsi::validate_ifc(ctx, instance) {
          Ok(instance) => Ok(Asymmetric::Ifc(instance)),
          Err(instance) => Err(Asymmetric::Ifc(instance)),
        },
      },
      Self::Cnsa => match algorithm {
        Asymmetric::Ecc(instance) => match Cnsa::validate_ecc(ctx, instance) {
          Ok(instance) => Ok(Asymmetric::Ecc(instance)),
          Err(instance) => Err(Asymmetric::Ecc(instance)),
        },
        Asymmetric::Ifc(instance) => match Cnsa::validate_ifc(ctx, instance) {
          Ok(instance) => Ok(Asymmetric::Ifc(instance)),
          Err(instance) => Err(Asymmetric::Ifc(instance)),
        },
      },
    }
  }
}

pub enum Status {
  Ok(PathBuf),
  Fail(PathBuf),
}

impl Termination for Status {
  fn report(self) -> std::process::ExitCode {
    match self {
      Self::Ok(_) => {
        println!("{}", &self);
        ExitCode::SUCCESS
      },
      Self::Fail(_) => {
        eprintln!("{}", &self);
        ExitCode::FAILURE
      },
    }
  }
}

impl fmt::Display for Status {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &self {
      Self::Ok(path) => write!(f, "ok: {}", path.display()),
      Self::Fail(path) => write!(f, "fail: {}", path.display()),
    }
  }
}

pub fn x509(ctx: &Context, path: &PathBuf, guide: &Guide, verbose: &bool) -> Status {
  let certificate = match Certificate::from_pem_file(path) {
    Ok(got) => got,
    Err(err) => {
      eprintln!("{}", err.to_string());
      return Status::Fail(path.to_path_buf());
    },
  };

  let mut pass = Status::Ok(path.to_path_buf());

  if let Some(got) = certificate.extract_hash_function() {
    match guide.validate_hash_function(ctx, got) {
      Ok(want) => {
        if *verbose {
          println!("hash function: got: {}, want: {}", got, want)
        }
      },
      Err(want) => {
        pass = Status::Fail(path.to_path_buf());
        eprintln!("hash function: got: {}, want: {}", got, want);
      },
    }
  }

  if let Some(got) = certificate.extract_signature_algorithm() {
    match guide.validate_signature_algorithm(ctx, got) {
      Ok(want) => {
        if *verbose {
          println!("signature algorithm: got: {}, want: {}", got, want)
        }
      },
      Err(want) => {
        pass = Status::Fail(path.to_path_buf());
        eprintln!("signature algorithm: got: {}, want: {}", got, want);
      },
    }
  }

  pass
}
