#![allow(dead_code)]

use common::{shape::Shape, shape_tree::ShapeTree};
use stream_executor::device_memory::DeviceMemoryBase;

// Class which encapsulates a buffer or set of buffers containing data of a
// particular Blitz shape.
pub struct ShapedBuffer {
  on_host_shape: Shape,
  on_device_shape: Shape,
  device_ordinal: i64,
  buffers: ShapeTree<DeviceMemoryBase>
}

impl ShapedBuffer {
  pub fn new() {}

  // Returns the shape of the on-host representation of the data held by this
  // ShapedBuffer.
  pub fn on_host_shape(&self) -> &Shape {
    &self.on_host_shape
  }

  // Returns the shape of the on-device representation of the data held by this
  // ShapedBuffer.
  pub fn on_device_shape(&self) -> &Shape {
    &self.on_device_shape
  }

  pub fn device_ordinal(&self) -> i64 {
    self.device_ordinal
  }

  pub fn root_buffer() {}
  pub fn buffer() {}
  pub fn set_buffer() {}
  pub fn set_buffers() {}
  pub fn set_shapes() {}

  // Returns the underlying ShapeTree containing all the device addresses in the
  // ShapedBuffer.
  pub fn buffers(&self) -> &ShapeTree<DeviceMemoryBase> {
    &self.buffers
  }

  pub fn sub_shaped_buffer() {}
  pub fn clear() {}

  pub fn to_string(&self) -> String {
    unimplemented!()
  }
}

// ScopedShapedBuffer takes allocated buffers as inputs, and deallocates on
// destruction. This class represents an owning wrapper around `ShapedBuffer`.
pub struct ScopedShapedBuffer {}

impl ScopedShapedBuffer {
  pub fn new() {}
}