use {
  self::directive::Directive,
  bitcoin::{
    opcodes,
    script::{self, Instruction},
    Transaction,
  },
  error::Error,
  serde::Serialize,
  std::fmt::{self, Display, Formatter},
};

pub use runestone::Runestone;

mod directive;
mod error;
mod runestone;
mod varint;

type Result<T, E = Error> = std::result::Result<T, E>;
