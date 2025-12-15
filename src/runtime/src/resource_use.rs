#![allow(dead_code)]

pub enum ResourceKind {
  // Side-effecting operations (i.e., infeed and outfeed) define their
  // execution order via token dependencies. We rely on token resource to
  // enforce ordering at run time.
  Token,
  // Collective operations must be scheduled in the same order as they are
  // defined in the HLO module. We rely on collective communicator resource
  // to enforce ordering at run time.
  CollectiveComunicator,
}