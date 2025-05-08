#![allow(dead_code)]

use common::{shape::{Shape, ShapeEqual}, shape_tree::ShapeTree, shape_util::ShapeUtil};
use stream_executor::device_memory::DeviceMemoryBase;

// Class which encapsulates a buffer or set of buffers containing data of a
// particular Blitz shape.
pub struct ShapedBuffer {
  on_host_shape: Shape,
  on_device_shape: Shape,
  device_ordinal: i64,
  physical_device_ordinal: i64,
  buffers: ShapeTree<DeviceMemoryBase>
}

impl ShapedBuffer {
  pub fn new(
    on_device_shape: &mut Shape,
    device_ordinal: i64,
    physical_device_ordinal: i64) -> Self
  {
    let mut phys_dev_ordinal = physical_device_ordinal;
    if phys_dev_ordinal == -1 {
      phys_dev_ordinal = device_ordinal;
    }
    let on_host_shape =
      ShapeUtil::device_shape_to_host_shape(on_device_shape.clone());

    ShapedBuffer {
      on_host_shape: on_host_shape,
      on_device_shape: on_device_shape.clone(),
      device_ordinal: device_ordinal,
      physical_device_ordinal: phys_dev_ordinal,
      buffers: ShapeTree::new(on_device_shape)
    }
  }

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

  // Return the root buffer of the shape (shape index {}).
  pub fn root_buffer(&self) -> &DeviceMemoryBase {
    self.buffer(0)
  }

  // Returns the buffer at the given shape index where index is defined as in
  // ShapeUtil::GetSubshape.
  pub fn buffer(&self, _index: usize) -> &DeviceMemoryBase {
    //self.buffers.element(index)
    unimplemented!()
  }

  // Sets the device memory buffer at the given index.
  pub fn set_buffer(&mut self, _buffer: DeviceMemoryBase, _index: usize) {
    //self.buffers.set_element_value(index, buffer);
  }

  // Sets all buffers.
  // Precondition: buffers.shape == on_device_shape_
  pub fn set_buffers(&mut self, buffers: ShapeTree<DeviceMemoryBase>) {
    assert!(ShapeEqual::new().equal(buffers.shape(), &self.on_device_shape));
    self.buffers = buffers;
  }

  // Reset the shape of this shaped buffer and underlying buffer structure.
  // Precondition: EqualStructure(this->on_device_shape_, on_device_shape).
  pub fn set_shapes(&mut self, on_device_shape: &Shape) {
    assert!(ShapeUtil::equal_structure(on_device_shape, &self.on_device_shape),
      "Structures are not the same.");
    self.on_host_shape = ShapeUtil::device_shape_to_host_shape(on_device_shape.clone());
    self.on_device_shape = on_device_shape.clone();
  }

  // Returns the underlying ShapeTree containing all the device addresses in the
  // ShapedBuffer.
  pub fn buffers(&self) -> &ShapeTree<DeviceMemoryBase> {
    &self.buffers
  }

  pub fn sub_shaped_buffer() {}

  // Set all device memory pointers in the object to null.
  pub fn clear(&mut self) {
    unimplemented!()
  }

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