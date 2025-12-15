#![allow(dead_code)]

// Execution graph defines the execution order of operations based on their
// buffer use and resource use dependencies.
//
// In Blitz:GPU and Blitz:CPU we compile HLO programs to a sequence of operations
// executed on the underlying device. Blitz compiler creates a sequential schedule
// that is used to assign buffers to operations. These operation can be
// implemented as thunks, or as commands (only on Blitz:GPU backend with CUDA
// graphs). Each operations reads and writes from/to buffer slices and uses
// resources (i.e. collective communicator).
//
// At run time we can relax sequential schedule and execute operations
// concurrently, as long as we don't create data races (reading and writing
// from/to the same or overlapping buffer slices concurrently), or resource
// races (using the same mutable resource concurrently).
//
// Resources can behave as buffers and require an execution order (operation
// must wait for the completion of execution of all dependencies), or as a
// scheduling barrier (operation must wait for the completion of scheduling of
// all dependencies). See more details in the `NodeEdge::Kind` definition.
//
// We use buffer and resource use conflicts to define an execution order of
// operations as a directed acyclic graph (DAG) that satisfies all dependencies.
//
// Backend-specific runtime relies on the execution graph to execute operations
// concurrently usult the underlying device concurrency mechanism, e.g.
// thread pools on CPU device, or CUDA streams on NVIDIA GPU device.

use std::io::Sink;

use crate::resource_use::ResourceKind;

pub struct ExecutionGraph {
  nodes_in_edges: Vec<NodeEdge>,
  nodes_out_edges: Vec<NodeEdge>,
  nodes_defs: Vec<NodeDef>,
  source: Vec<i64>,
  sink: Vec<i64>,
  is_sequential: bool,
}

impl ExecutionGraph {
  // Returns the registered renderer for execution graphs.
  pub fn get_renderer() -> &Renderer {
    unimplemented!()
  }

  // Registers a renderer for execution graphs.
  pub fn register_renderer(renderer: &Renderer) {
    unimplemented!()
  }

  // Constructs an execution graph from a sequence of operations.
  pub fn create(operations: &Vec<Operation>) -> Result<Self, String> {
    // Make sure that operations sequence size fits into NodeId.
    if operations.len() > i64::MAX {
      let mut err_msg =
        "Can't create ExecutionGraph for more than ".to_string();
      err_msg.push(&operations.len().to_string());
      err_msg.push(" operations");
      return Err(err_msg);
    }

    let mut builders: Vec<NodeDefBuilder> = vec![];
    let mut buffer_rwsets = vec![];
    let mut resource_rwsets = vec![];

    for i in 0..operations.len() {
      builders[i].id = i as i64;
    }
  }

  // Returns execution graph nodes definitions.
  pub fn node_defs(&self) -> &Vec<NodeDef> {
    &self.nodes_defs
  }

  // Source nodes are the nodes that do not have any in-edges.
  pub fn source(&self) -> &Vec<i64> {
    &self.source
  }

  // Sink nodes are the nodes that do not have any out-edges.
  pub fn sink(&self) -> &Vec<i64> {
    &self.sink
  }

  // Returns true if a given node id is a source node.
  pub fn is_source(&self, id: i64) -> bool {
    for src_id in self.source {
      if id == src_id { return true; }
    }
    false
  }

  // Returns true if a given node id is a sink node.
  pub fn is_sink(&self, id: i64) -> bool {
    for sink_id in self.sink {
      if id == sink_id { return true; }
    }
    false
  }

  // Returns in-edges for a given node id.
  pub fn in_edges(&self, id: i64) -> &Vec<NodeEdge> {
    debug_assert!(id == self.nodes_defs[id as usize].id);
    &self.nodes_defs[id as usize].in_edges
  }

  // Returns out-edges for a given node id.
  pub fn out_edges(&self, id: i64) -> &Vec<NodeEdge> {
    debug_assert!(id == self.nodes_defs[id as usize].id);
    &self.nodes_defs[id as usize].out_edges
  }

  // Returns priority for a given node id.
  pub fn priority(&self, id: i64) -> i64 {
    debug_assert!(id == self.nodes_defs[id as usize],id);
    self.nodes_defs[id as usize].priority
  }

  pub fn is_sequential(&self) -> bool {
    self.is_sequential
  }

  fn create_node_defs(
    &self,
    builders: &Vec<NodeDefBuilder>) -> (Vec<NodeEdge>, Vec<NodeEdge>, Vec<NodeDef>)
  {
    let mut nodes_in_edges = vec![];
    let mut nodes_out_edges = vec![];
    let mut nodes_defs = vec![];

    for b in builders {
      let num_in_edges = b.in_edges.len();
      let num_out_edges = b.out_edges.len();

      let mut inserted_in_edges = vec![];
      for in_edge in &b.in_edges {
        inserted_in_edges.push(in_edge.clone());
      }
      let mut inserted_out_edges = vec![];
      for out_edge in &b.out_edges {
        inserted_out_edges.push(out_edge.clone());
      }
      let node_def =
        NodeDef::new(b.id, inserted_in_edges, inserted_out_edges, b.priority);
      nodes_defs.push(node_def);
    }

    (nodes_in_edges, nodes_out_edges, nodes_defs)
  }

  fn erase_edge(
    &self,
    from: &NodeDefBuilder,
    to: &NodeDefBuilder,
    kind: NodeEdgeKind) -> i64
  {
    debug_assert!(from.id != to.id, "Nodes must be different");
    debug_assert!(from.id < to.id, "Nodes must be ordered");

    // Short-circuit if out or in-edges are empty.
    if from.out_edges.is_empty() || to.in_edges.is_empty() {
      // TODO
      return 0;
    }
    // Short-circuit if out-edges or in-edges don't intersect with `to` or `from`
    // node ids (remember that edges are sorted).
    if from.out_edges.last().unwrap().id < to.id ||
      to.in_edges.first().unwrap().id > from.id
    {
      // TODO
      return 0;
    }
    // Comparator to find a node edge with a given node id.
    let less_than = |edge: &NodeEdge, id: i64| -> bool {
      edge.id < id
    };
    // Check if `from` node has an out edge to `to` node.
    let mut out_edges_iter = vec![];
    let mut has_out_edge = false;
    let mut target_out_edge = None;
    for out_edge in &from.out_edges {
      if out_edge.id >= to.id { out_edges_iter.push(out_edge.clone()); } 
    }
    if !out_edges_iter.is_empty() {
      for out_edge in &out_edges_iter {
        if out_edge.id == to.id {
          has_out_edge = true;
          target_out_edge = Some(out_edge.clone());
        }
      }
    }
    // Short-circuit if there is no out edge from `from` node to `to` node.
    if !has_out_edge {
      // TODO
      return 0;
    }
    // Check if `to` node has an in edge from `from` node.
    let mut in_edges_iter = vec![];
    let mut has_in_edge = false;
    let mut target_in_edge = None;
    for in_edge in &to.in_edges {
      if in_edge.id >= from.id { in_edges_iter.push(in_edge.clone()); }
    }
    if !in_edges_iter.is_empty() {
      for in_edge in &in_edges_iter {
        if in_edge.id == from.id {
          has_in_edge = true;
          target_in_edge = Some(in_edge.clone());
        }
      }
    }
    debug_assert!(has_in_edge, "In-edge must exist if out-edge exists");
    debug_assert_eq!(target_in_edge.as_ref().unwrap().kind,
      target_out_edge.as_ref().unwrap().kind);

    // TODO
    
    // We can't erase an edge with a stronger ordering guarantee.
    if target_in_edge.as_ref().unwrap().kind == NodeEdgeKind::Execution &&
      kind -= NodeEdgeKind::Scheduling
    {
      return 0;
    }
    // We erased exactly one edge between `from` and `to` nodes.
    from.out_edges.remove(target_out_edge.unwrap());
    to.in_edges.remove(target_in_edge.unwrap());
    return 1;
  }
}

// A base class for an operation that can be executed by the runtime.
pub struct Operation {
  named_nested_operations: Vec<(String, Vec<Operation>)>
}

impl Operation {
  pub fn default() -> Self {
    Operation { named_nested_operations: Vec::new() }
  }

  pub fn name(&self) -> String {
    unimplemented!()
  }

  pub fn op_type_id(&self) -> i64 {
    unimplemented!()
  }

  pub fn buffer_uses(&self) {
      
  }

  pub fn resource_uses(&self) {
      
  }
}

// Edge kind defines execution ordering between two operations. Scheduling
// edge is weaker than an execution edge, as it gives more flexibility
// to the backend runtime to execute operations concurrently.
#[derive(Debug)]
pub enum NodeEdgeKind {
  // If two operations have a scheduling edge between them, then the
  // dependent operation must be scheduled (start execution) after the
  // dependency operation scheduled (started execution), however it doesn't
  // have to wait for the completion of execution. We use this type of
  // edge to guarantee that operations that share the same resource (i.e.
  // collective communicator) start execution in a deterministic order
  // across different ranks, however the execution of operations can
  // overlap and finish in any order, and backend-implementation specific.
  Scheduling,
  // If two operations have an execution edge between them, then the
  // dependent operation must wait for the completion of dependency
  // operation execution. We use this type of edge to order execution of
  // operations that read and write from/to the same buffers, as otherwise
  // we may create data races.
  Execution,
}

// An edge between two nodes created for the execution graph operations.
#[derive(Debug, Clone, PartialEq)]
pub struct NodeEdge {
  kind: NodeEdgeKind,
  id: i64
}

impl NodeEdge {
  pub fn kind_of(resource: ResourceKind ) -> NodeEdgeKind {
    match resource {
      ResourceKind::Token => NodeEdgeKind::Execution,
      ResourceKind::CollectiveComunicator => NodeEdgeKind::Scheduling
    }
  }

  pub fn absl_stringify(&self, _sink: &Sink, _kind: NodeEdgeKind) {
    unimplemented!()
  }
}

// NodeDef defines a dependency-based execution order for all operations.
pub struct NodeDef {
  id: i64,
  in_edges: Vec<NodeEdge>,
  out_edges: Vec<NodeEdge>,
  // When doing the transitive reduction, we assign a priority to each node
  // based on the number of nodes that are reachable from the given node. The
  // assumption is that by executing nodes with higher priority first we will
  // unlock more nodes for execution.
  priority: i64,
}

impl NodeDef {
  pub fn new(
    id: i64,
    in_edges: Vec<NodeEdge>,
    out_edges: Vec<NodeEdge>,
    priority: i64) -> Self
  {
    NodeDef {
      id: id,
      in_edges: in_edges,
      out_edges: out_edges,
      priority: priority
    }    
  }
}

// A NodeDef builder to collect all in-edges and out-edges before constructing
// a NodeDef. We use it at dependency graph construction time when we don't
// know how many in-edges and out-edges we have in total.
pub struct NodeDefBuilder {
  id: i64,
  priority: i64,
  in_edges: Vec<NodeEdge>,
  out_edges: Vec<NodeEdge>,
}

pub struct Renderer {}

impl Renderer {
  pub fn default() -> Self {
    Renderer {  }
  }

  // Generates a string representation for the given execution graph
  // operations which can be published to a URL using `PublishGraph`.
  pub fn generate_graph_as_string(&self, _pperations: &Vec<Operation>) -> String {
    unimplemented!()
  }

  // Publishes the generated graph.
  pub fn publish_graph(&self, _graph_as_string: String) -> Result<String, String> {
    unimplemented!()
  }
}