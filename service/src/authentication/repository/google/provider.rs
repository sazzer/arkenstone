/// Representation of a Provider for authenticating with Google
pub struct Provider {}

impl crate::authentication::repository::Provider for Provider {}

#[cfg(test)]
mod tests {
  use super::*;
  use std::boxed::Box;

  #[test]
  fn test() {
    let _b: Box<dyn crate::authentication::repository::Provider> = Box::new(Provider {});
  }
}
