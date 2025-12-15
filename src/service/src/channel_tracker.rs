use std::sync::Mutex;
use common::blitz_data::{ChannelHandle, ChannelType};
use num::ToPrimitive;

// Tracks channels between computations in the Blitz service. Channels
// are associated with a unique handle and can be resolved from the handle for
// later use.
pub struct ChannelTracker {
  // The next sequence number to assign to a channel.
  next_channel: Mutex<i64>,
}

impl ChannelTracker {
  pub fn default() -> Self {
    ChannelTracker { next_channel: Mutex::new(1) }
  }

  // Creates a new Channel object and returns the corresponding
  // ChannelHandle for it.
  pub fn new_channel(&mut self, t: ChannelType) -> Result<ChannelHandle, String> {
    if t == ChannelType::Invalid {
      let err_msg = "Invalid channel type.".to_string();
      return Err(err_msg);
    }
    let new_handle = ChannelHandle::new(
      t, self.next_channel.lock().unwrap().to_i64().unwrap());
    let _ = self.next_channel.lock().unwrap().checked_add(1);
    Ok(new_handle)
  }
}