pub mod lookup;
mod resolver;
pub use resolver::*;

pub fn lookup_and_load<P>(dir_path: P) -> Result<crate::context::Context, crate::error::Error>
where
  P: AsRef<std::path::Path>,
{
  let path = lookup::dir(dir_path)?;
  resolver::load(path.as_path())
}
