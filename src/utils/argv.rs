use std::collections::HashMap;

pub fn parse<I, J>(args: I) -> (Vec<String>, HashMap<String, String>)
where
  J: Into<String>,
  I: Iterator<Item = J>,
{
  let matches = vec![("-", 1), ("--", 2)];

  let mut params: Vec<String> = Vec::new();
  let mut vars: HashMap<String, String> = HashMap::new();

  let mut tmp_key: Option<String> = None;

  for arg in args {
    let arg: String = arg.into();

    let (mtch0, len0) = matches[0];

    if arg.len() >= len0 + 1 && &arg[..len0] == mtch0 {
      if let Some(key) = tmp_key.take() {
        vars.insert(key, "true".to_string());
      }

      let mut key: &str = &arg[len0..];
      let (mtch1, len1) = matches[1];

      if arg.len() >= len1 + 1 && &arg[..len1] == mtch1 {
        key = &arg[len1..];
      }

      let mut s: Vec<&str> = key.split("=").collect();

      if s.len() >= 2 {
        key = s.remove(0);
        vars.insert(key.to_string(), s.join("="));
      } else {
        tmp_key = Some(s[0].into());
      }

      continue;
    }

    if let Some(key) = tmp_key.take() {
      vars.insert(key, arg);
    } else {
      params.push(arg);
    }
  }

  if let Some(key) = tmp_key.take() {
    vars.insert(key, "true".to_string());
  }

  (params, vars)
}

pub fn extract_vars(map: &HashMap<String, String>) -> HashMap<String, String> {
  let mtch = "var.";
  let len = mtch.len();
  let mut vars: HashMap<String, String> = HashMap::new();

  for (key, value) in map.iter() {
    if key.len() > len {
      vars.insert((&key[len..]).to_string(), value.to_string());
    }
  }

  vars
}
