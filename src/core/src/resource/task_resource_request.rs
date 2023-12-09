#![allow(dead_code)]

pub struct TaskResourceRequest {
  resource_name: String,
  amount: f64,
}

impl TaskResourceRequest {
  pub fn new(resource_name: String, amount: f64) -> Self {
    TaskResourceRequest {
      resource_name: resource_name,
      amount: amount
    }
  }

  pub fn hash_code(&self) {}

  pub fn to_string(&self) -> String {
    String::from("name:")
      + self.resource_name.as_str()
      + ", amount"
      + self.amount.to_string().as_str()
  }
}