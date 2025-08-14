#![allow(dead_code)]

use stream_executor::{
  platform::StreamPriority,
  stream::Stream,
  stream_executor::StreamExecutor
};

// Pool of stream_executor::Streams, which are created as needed and
// destroyed when the pool is destroyed.
pub struct StreamPool {
  streams_default: Vec<Stream>,
  streams_lowest: Vec<Stream>,
  streams_highest: Vec<Stream>
}

impl StreamPool {
  pub fn new(_executor: StreamExecutor) -> Self {
    StreamPool {
      streams_default: Vec::new(),
      streams_lowest: Vec::new(),
      streams_highest: Vec::new()
    }
  }

  // Returns a pointer to a stream in the pool, creating a new stream
  // if none are available in the pool. The returned smart pointer
  // returns the stream to the pool on destruction.
  //
  // This method is thread-safe.
  pub fn borrow_stream(_priority: StreamPriority) {
    unimplemented!()
  }

  // Puts a pointer to a stream back into the pool, leaving it free
  // for future use. Streams that have previously encountered errors
  // are deleted, and not returned to the pool.
  //
  // This method is thread-safe.
  fn return_stream(&mut self, stream: Stream) {
    if stream.ok() {
      println!("StreamPool returning ok stream ({:?})", stream);
      // TODO: lock
      let priority = stream.priority();
      match priority {
        StreamPriority::Default => self.streams_default.push(stream),
        StreamPriority::Lowest => self.streams_lowest.push(stream),
        StreamPriority::Highest => self.streams_highest.push(stream),
      };
    } else {
      // If the stream has encountered any errors, all subsequent operations on it
      // will fail. So just delete the stream, and rely on new streams to be
      // created in the future.
      println!("StreamPool deleting !ok stream ({:?})", stream);
    }
  }
}