#[cfg(test)]
mod tests {
  #[test]
  fn lookup() {
    let mut dir_path = std::env::current_dir().unwrap();
    dir_path.push("tmp");
    let res = crate::importer::lookup::dir(dir_path);
    println!("{:?}", res);
  }

  #[test]
  fn parse_file() -> Result<(), crate::error::Error> {
    futures::executor::block_on(parse_file_async())
  }

  async fn parse_file_async() -> Result<(), crate::error::Error> {
    use crate::utils::path::PathExt;
    let path: std::path::PathBuf = std::path::Path::new("./")
      .join("tmp")
      .join("simple.yml")
      .normalize();

    let context = crate::importer::load(&path)?;
    // println!("{:?}", context);
    context.run("how", None).await?;
    // context.run("ls", None).await;

    Ok(())
  }

  #[test]
  fn parse_arguments() -> Result<(), crate::error::Error> {
    // let args = std::env::args();
    let args = vec![
      "wk:hello",
      "--var.buddy=\"john\"",
      "--var.buddy0=",
      "--var.greeting",
      "hello",
      "--var.debug",
      "-d",
      "--var0",
      "-var1",
      "--var2.=",
      "--v=toto=plouf",
    ];
    println!("{:?}", args.join(" "));
    let res = crate::utils::argv::extract_vars(args.into_iter());
    println!("{:?}", res);
    // let vars = crate::utils::argv::extract_vars(&argv);
    // println!("{:?}", vars);
    Ok(())
  }
}
