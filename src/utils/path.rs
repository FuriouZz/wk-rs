pub trait PathExt {
  fn normalize<S>(&self) -> S
  where
    S: std::convert::From<std::path::PathBuf>;

  fn to_slash(&self) -> Option<String>;
}

impl PathExt for std::path::PathBuf {
  fn normalize<S>(&self) -> S
  where
    S: std::convert::From<std::path::PathBuf>,
  {
    let mut nbuf = std::path::PathBuf::new();

    for c in self.components() {
      nbuf = nbuf.join(c);
    }

    return nbuf.into();
  }

  #[cfg(not(target_os = "windows"))]
  fn to_slash(&self) -> Option<String> {
    self.to_str().map(str::to_string)
  }

  #[cfg(target_os = "windows")]
  fn to_slash(&self) -> Option<String> {
    let components = self
      .components()
      .map(|c| match c {
        std::path::Component::RootDir => Some(""),
        std::path::Component::CurDir => Some("."),
        std::path::Component::ParentDir => Some(".."),
        std::path::Component::Prefix(ref p) => p.as_os_str().to_str(),
        std::path::Component::Normal(ref p) => p.to_str(),
      })
      .collect::<Option<Vec<_>>>();

    components.map(|v| {
      if v.len() == 1 && v[0].is_empty() {
        // Special case for '/'
        "/".to_string()
      } else {
        v.join("/")
      }
    })
  }
}

impl PathExt for std::path::Path {
  fn normalize<S>(&self) -> S
  where
    S: std::convert::From<std::path::PathBuf>,
  {
    let mut nbuf = std::path::PathBuf::new();

    for c in self.components() {
      nbuf = nbuf.join(c);
    }

    return nbuf.into();
  }

  #[cfg(not(target_os = "windows"))]
  fn to_slash(&self) -> Option<String> {
    self.to_str().map(str::to_string)
  }

  #[cfg(target_os = "windows")]
  fn to_slash(&self) -> Option<String> {
    let components = self
      .components()
      .map(|c| match c {
        std::path::Component::RootDir => Some(""),
        std::path::Component::CurDir => Some("."),
        std::path::Component::ParentDir => Some(".."),
        std::path::Component::Prefix(ref p) => p.as_os_str().to_str(),
        std::path::Component::Normal(ref p) => p.to_str(),
      })
      .collect::<Option<Vec<_>>>();

    components.map(|v| {
      if v.len() == 1 && v[0].is_empty() {
        // Special case for '/'
        "/".to_string()
      } else {
        v.join("/")
      }
    })
  }
}
