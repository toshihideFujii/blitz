#![allow(dead_code)]

pub trait ConfigProvider {
  fn get(key: String) -> Option<String>;
}

pub trait EnvProvider {
  fn get(key: String) -> Option<String>;
}

pub trait SystemProvider {
  fn get(key: String) -> Option<String>;
}

pub trait MapProvider {
  fn get(key: String) -> Option<String>;
}