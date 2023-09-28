use super::*;

#[derive(Default, Serialize, Debug, PartialEq)]
pub struct Runestone {
  pub directives: Vec<Directive>,
  pub decimals: Option<u128>,
  pub symbol: Option<u128>,
}
