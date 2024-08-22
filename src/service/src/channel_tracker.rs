#![allow(dead_code)]

use common::blitz_data::{ChannelHandle, ChannelType};

// Tracks channels between computations in the Blitz service. Channels
// are associated with a unique handle and can be resolved from the handle for
// later use.
pub struct ChannelTracker {}

impl ChannelTracker {
  pub fn new() -> Self {
    ChannelTracker {  }
  }

  // Creates a new Channel object and returns the corresponding
  // ChannelHandle for it.
  pub fn new_channel(&self, _t: ChannelType) -> Result<ChannelHandle, String> {
    unimplemented!()
  }
}