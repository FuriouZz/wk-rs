use std::collections::HashMap;

const MATCHES: [&'static str; 5] = ["--", "-", "--var.", "=", "true"];

pub fn match_pattern<'a>(s: &'a str, pattern: &str) -> Option<(&'a str, &'a str)> {
  let len = pattern.len();
  if s.len() >= len + 1 {
    unsafe {
      if s.get_unchecked(0..len) == pattern {
        let matched = s.get_unchecked(..len);
        if matched.is_empty() {
          return None;
        }
        let rest = s.get_unchecked(len..);
        return Some((matched, rest));
      }
    }
  }
  None
}

pub fn is_option(s: &str) -> bool {
  for m in &MATCHES[..2] {
    let res = match_pattern(s, m);
    if res.is_some() {
      return true;
    }
  }
  return false;
}

pub fn get_option(s: &str) -> Option<(&str, &str)> {
  for m in &MATCHES[..2] {
    let res = match_pattern(s, m);
    if res.is_some() {
      return res;
    }
  }
  return None;
}

pub fn is_bool_option(s: &str) -> bool {
  let res0 = match_pattern(s, MATCHES[0]);
  let res1 = match_pattern(s, MATCHES[1]);
  return res0.is_none() && res1.is_some();
}

pub fn get_bool_option(s: &str) -> Option<(&str, &str)> {
  let res0 = match_pattern(s, MATCHES[0]);
  let res1 = match_pattern(s, MATCHES[1]);
  if res0.is_none() && res1.is_some() {
    return res1;
  }
  None
}

pub fn is_var_option(s: &str) -> bool {
  match_pattern(s, MATCHES[2]).is_some()
}

pub fn get_var_option(s: &str) -> Option<(&str, &str)> {
  match_pattern(s, MATCHES[2])
}

pub fn extract_vars<'a, I>(args: I) -> (Vec<&'a str>, HashMap<&'a str, &'a str>)
where
  I: Iterator<Item = &'a str>,
{
  let mut params: Vec<&str> = Vec::new();
  let mut vars: HashMap<&str, &str> = HashMap::new();

  let mut tmp_key: Option<&str> = None;

  for arg in args {
    if let Some(key) = tmp_key.take() {
      if !is_option(&arg) {
        vars.insert(key, &arg);
        continue;
      } else {
        vars.insert(key, MATCHES[4]);
      }
    }

    if let Some(v) = get_var_option(&arg) {
      let mut split: Vec<&str> = v.1.split(MATCHES[3]).collect();
      if split.len() >= 2 {
        let key = split.remove(0);

        if let Some(value) = match_pattern(v.1, key) {
          if !key.is_empty() && !value.1.is_empty() {
            if let Some(val) = match_pattern(value.1, MATCHES[3]) {
              vars.insert(key, val.1);
              continue;
            }
          }
        }
      } else {
        tmp_key = Some(v.1);
        continue;
      }
    }

    params.push(&arg);
  }

  if let Some(key) = tmp_key.take() {
    vars.insert(key, MATCHES[4]);
  }

  (params, vars)
}

pub fn extract_option<'a, I>(args: I) -> (Vec<&'a str>, HashMap<&'a str, &'a str>)
where
  I: Iterator<Item = &'a str>,
{
  let mut params: Vec<&str> = Vec::new();
  let mut vars: HashMap<&str, &str> = HashMap::new();

  let mut tmp_key: Option<&str> = None;

  for arg in args {

    if let Some(key) = tmp_key.take() {
      if !is_option(&arg) {
        vars.insert(key, &arg);
        continue;
      } else {
        vars.insert(key, MATCHES[4]);
      }
    }

    if let Some(v) = get_option(&arg) {
      if let Some(bool_option) = get_bool_option(&arg) {
        vars.insert(bool_option.1, MATCHES[4]);
        continue;
      }

      let mut split: Vec<&str> = v.1.split(MATCHES[3]).collect();
      if split.len() >= 2 {
        let key = split.remove(0);
        if let Some(value) = match_pattern(v.1, key) {
          if !key.is_empty() && !value.1.is_empty() {
            if let Some(val) = match_pattern(value.1, MATCHES[3]) {
              vars.insert(key, val.1);
              continue;
            }
          }
        }
      } else {
        tmp_key = Some(v.1);
        continue;
      }
    }

    params.push(&arg);
  }

  if let Some(key) = tmp_key.take() {
    vars.insert(key, MATCHES[4]);
  }

  (params, vars)
}

pub fn extract_vars_from_args(argv: std::env::Args) -> (Vec<String>, HashMap<String, String>)
{
  let mut args: Vec<String> = Vec::new();
  for arg in argv {
    args.push(arg);
  }
  let args = args.join(" ");
  let split = args.as_str().split(" ");
  let (params, options) = extract_vars(split);

  let params_string: Vec<String> = params.iter().map(|s| s.to_string()).collect();
  let mut options_string: HashMap<String, String> = HashMap::new();

  for option in options {
    options_string.insert(option.0.to_string(), option.1.to_string());
  }

  (params_string, options_string)
}

pub fn extract_option_from_args(argv: std::env::Args) -> (Vec<String>, HashMap<String, String>)
{
  let mut args: Vec<String> = Vec::new();
  for arg in argv {
    args.push(arg);
  }
  let args = args.join(" ");
  let split = args.as_str().split(" ");
  let (params, options) = extract_option(split);

  let params_string: Vec<String> = params.iter().map(|s| s.to_string()).collect();
  let mut options_string: HashMap<String, String> = HashMap::new();

  for option in options {
    options_string.insert(option.0.to_string(), option.1.to_string());
  }

  (params_string, options_string)
}