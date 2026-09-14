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

use std::{cmp::Ordering, io::Sink};

use crate::{buffer_use::{BufferReadWriteSet, BufferUse},
  resource_use::{ResourceKind, ResourceReadWriteSet, ResourceUse}};

// A helper function to create a predicate that checks if a given node edge
// points to a given node id.
fn edge_predicate(id: i64, edge: &NodeEdge) -> bool {
  edge.id == id
}

// If any of the resource uses requires execution edge, we return kExecution
// edge kind, otherwise we return kScheduling edge kind.
fn edge_kind(resource_uses: &Vec<ResourceUse>) -> NodeEdgeKind {
  let requires_execution_edge =
    |resource_use: &ResourceUse| -> bool
  {
    let kind = resource_use.resource().kind();
    NodeEdge::kind_of(kind) == NodeEdgeKind::Execution
  };
  for resource_use in resource_uses {
    if requires_execution_edge(resource_use) { return NodeEdgeKind::Execution; }
  }
  NodeEdgeKind::Scheduling
}

pub struct ExecutionGraph {
  nodes_in_edges: Vec<NodeEdge>,
  nodes_out_edges: Vec<NodeEdge>,
  nodes_defs: Vec<NodeDef>,
  source: Vec<i64>,
  sink: Vec<i64>,

  // If NodeDef graph dependency structure is sequential and does not have any
  // opportunities for executing operations concurrently. BLITZ runtime can use
  // this property of the execution graph to skip expensive async execution and
  // simply run all operations one by one.
  is_sequential: bool,
}

impl ExecutionGraph {
  // Returns the registered renderer for execution graphs.
  pub fn get_renderer() -> Renderer {
    unimplemented!()
  }

  // Registers a renderer for execution graphs.
  pub fn register_renderer(_renderer: &Renderer) {
    unimplemented!()
  }

  pub fn new(
    nodes_in_edges: Vec<NodeEdge>,
    nodes_out_edges: Vec<NodeEdge>,
    nodes_defs: Vec<NodeDef>) -> Self
  {
    let mut execution_graph = ExecutionGraph {
      nodes_in_edges,
      nodes_out_edges,
      nodes_defs,
      source: Vec::new(),
      sink: Vec::new(),
      is_sequential: true
    };
    // Identify source and sink nodes in the execution graph.
    for i in 0..execution_graph.nodes_defs.len() {
      // Mark nodes with empty in-edges as source nodes.
      if execution_graph.nodes_defs[i].in_edges.is_empty() {
        execution_graph.source.push(i as i64);
      }
      // Mark nodes with empty out-edges as sink nodes.
      if execution_graph.nodes_defs[i].out_edges.is_empty() {
        execution_graph.sink.push(i as i64);
      }
    }
    // Check if constructed execution DAG is sequential: every node depends on the
    // completion of the previous node.
    for i in 1..execution_graph.nodes_defs.len() {
      if !execution_graph.is_sequential { break; }
      let mut count = 0;
      for edge in &execution_graph.nodes_defs[i].in_edges {
        if edge.id == (i as i64) - 1 { count += 1; }
      }
      if count == 0 { execution_graph.is_sequential = false; }
    }

    println!("Constructed execution graph with {:?} nodes:
      #source_nodes={:?} #sink_nodes={:?} is_sequential={:?}",
      execution_graph.nodes_defs, execution_graph.source.len(),
      execution_graph.sink.len(), execution_graph.is_sequential);
    
    // Sanity check that all vectors are empty or all vectors are non-empty.
    debug_assert!((!execution_graph.source.is_empty() && !execution_graph.sink.is_empty()) ||
      (execution_graph.source.is_empty() || execution_graph.sink.is_empty()));
    
    execution_graph    
  }

  // Constructs an execution graph from a sequence of operations.
  pub fn new_from_operations(operations: &Vec<Operation>) -> Result<Self, String> {
    // Make sure that operations sequence size fits into NodeId.
    if operations.len() > i64::MAX as usize {
      let mut err_msg =
        "Can't create ExecutionGraph for more than ".to_string();
      err_msg.push_str(&operations.len().to_string());
      err_msg.push_str(" operations");
      return Err(err_msg);
    }

    let mut builders: Vec<NodeDefBuilder> =
      vec![NodeDefBuilder::default(); operations.len()];
    let mut buffer_rwsets: Vec<BufferReadWriteSet> =
      vec![BufferReadWriteSet::default(); operations.len()];
    let mut resource_rwsets: Vec<ResourceReadWriteSet> =
      vec![ResourceReadWriteSet::default(); operations.len()];

    for i in 0..operations.len() {
      println!("Processing operation {:?}", operations[i].to_string());
      builders[i].id = i as i64;

      let op = &operations[i];
      buffer_rwsets[i].add_all(op.buffer_uses());
      resource_rwsets[i].add_all(op.resource_uses());

      for j in 0..i {
        if buffer_rwsets[j].has_conflicts_by_other(&buffer_rwsets[i]) {
          // If we have buffer conflicts we must add an execution edge to
          // guarantee that we don't have data races at run time.
          builders[j].out_edges.push(NodeEdge { kind: NodeEdgeKind::Execution, id: i as i64 });
          builders[i].in_edges.push(NodeEdge { kind: NodeEdgeKind::Execution, id: j as i64});
        } else if resource_rwsets[j].has_conflicts_by_other(&resource_rwsets[i]) {
          // If we have resource conflicts, we must check resources that are
          // accessed by both nodes to find out what kind of edge we need to add.
          let kind = edge_kind(
            &resource_rwsets[j].conflicts(&resource_rwsets[i]));
          builders[j].out_edges.push(NodeEdge { kind: kind.clone(), id: i as i64 });
          builders[i].in_edges.push(NodeEdge { kind: kind.clone(), id: j as i64 });
          println!("Added edge {:?} -> {:?} {:?}", j, i, kind);
        }
      }
    }

    // Verify that both in-edges and out-edges are sorted in ascending order
    // according to node id as we use this property later.
    for i in 0..builders.len() {
      let by_id =
        |a: &NodeEdge, b: &NodeEdge| -> Ordering
      {
        if a.id < b.id {
          return Ordering::Less;
        } else if a.id == b.id {
          return Ordering::Equal;
        } else {
          return Ordering::Greater;
        }
      };
      builders[i].out_edges.sort_by(by_id);
      builders[i].in_edges.sort_by(by_id);
    }

    // Erase redundant edges between nodes.
    let num_erased_edges =
      ExecutionGraph::run_transitive_reduction_and_update_priorities(&mut builders);
    println!("Transitive reduction erased {:?} edges from the execution graph", num_erased_edges);

    let node_defs =
      ExecutionGraph::create_node_defs(&builders);
    Ok(ExecutionGraph::new(
      node_defs.0, node_defs.1, node_defs.2))
  }

  fn run_transitive_reduction_and_update_priorities(
    builders: &mut Vec<NodeDefBuilder>) -> i64
  {
    let mut num_erapsed_edges = 0;

    // Keep workspace for DFS traversal between iterations.
    let mut state =
      TransitiveResuctionDfsState::default();

    // For each node we do a DFS traversal and delete redundant edges that
    // connect source node with the node reachable via DFS. We do traversal in
    // reverse order as we end up traversing fewer edges this way.
    for i in (0..builders.len()).rev() {
      //slet builders_len = builders.len();
      let mut source_node = builders[i].clone();

      // Clear DFS state from previous iteration.
      state.clear(builders.len());

      let mut out_edges = vec![];
      out_edges.clone_from(&source_node.out_edges);
      for out_edge in &out_edges {
        debug_assert!(state.empty(), "Stack must be empty at the start of the DFS");

        // Initialize state with nodes reachable via `out_edge`. We mark immediate
        // out nodes as visited to correctly compute node priority below.
        let out_node = builders[out_edge.id as usize].clone();
        state.visited(out_edge.id);
        let mut target_out_edges = vec![];
        target_out_edges.clone_from(&out_node.out_edges);
        state.push_to_stack_edges(target_out_edges);

        // Do a round of DFS traversal and delete redundant edges from the
        // `source_node` to the nodes reachable via DFS.
        while !state.empty() {
          let node_edge = state.pop_from_stack();
          let mut node = builders[node_edge.id as usize].clone();

          // If we reached `node` via a scheduling edge, then we can't remove an
          // execution edge from the `source_node`, as we might weaker the
          // execution order and introduce a data race.
          let has_scheduling_edge =
            out_edge.kind == NodeEdgeKind::Scheduling ||
            node_edge.kind == NodeEdgeKind::Scheduling ||
            state.num_scheduling_edges();
          let mut kind = NodeEdgeKind::Execution;
          if has_scheduling_edge {
            kind = NodeEdgeKind::Scheduling;
          }
          num_erapsed_edges +=
            ExecutionGraph::erase_edge(&mut source_node, &mut node, kind);

          // Keep following nodes reachable via `node` out edges.
          let mut node_out_edges = vec![];
          node_out_edges.clone_from(&node.out_edges);
          state.push_to_stack_edges(node_out_edges);
          builders[node_edge.id as usize] = node;
        }
        builders[out_edge.id as usize] = out_node;
      }

      // Set node priority to the number of visited nodes in the DFS traversal.
      // !!!!! CAUTION !!!!!
      // source_node.out_edges is changed in erase_edge(), but out_edges is not changed.
      // so don't set out_edges to source_node.out_edges.
      //source_node.out_edges = out_edges; // <- Don't do this.
      source_node.priority = state.num_visited();
      builders[i] = source_node;
    }

    num_erapsed_edges  
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
    for src_id in &self.source {
      if id == *src_id { return true; }
    }
    false
  }

  // Returns true if a given node id is a sink node.
  pub fn is_sink(&self, id: i64) -> bool {
    for sink_id in &self.sink {
      if id == *sink_id { return true; }
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
    debug_assert!(id == self.nodes_defs[id as usize].id);
    self.nodes_defs[id as usize].priority
  }

  pub fn is_sequential(&self) -> bool {
    self.is_sequential
  }

  fn create_node_defs(
    builders: &Vec<NodeDefBuilder>) -> (Vec<NodeEdge>, Vec<NodeEdge>, Vec<NodeDef>)
  {
    // Find how many in-edges and out-edges we have in total.
    let mut num_in_edges = 0;
    let mut num_out_edges = 0;
    for b in builders {
      num_in_edges += b.in_edges.len();
      num_out_edges += b.out_edges.len();
    }

    // Reserve memory to avoid re-allocation and dangling spans into freed memory.
    let mut nodes_in_edges = vec![];
    nodes_in_edges.reserve(num_in_edges);
    let mut nodes_out_edges = vec![];
    nodes_out_edges.reserve(num_out_edges);
    let mut nodes_defs = vec![];
    nodes_defs.reserve(builders.len());

    for b in builders {
      let num_in_edges = b.in_edges.len();
      let num_out_edges = b.out_edges.len();

      let mut inserted_in_edges = vec![];
      for in_edge in &b.in_edges {
        nodes_in_edges.push(in_edge.clone());
      }
      if num_in_edges != 0 {
        let start = nodes_in_edges.len() - b.in_edges.len();
        let end = start + num_in_edges;
        for i in start..end {
          inserted_in_edges.push(nodes_in_edges[i].clone());
        }
      }

      let mut inserted_out_edges = vec![];
      for out_edge in &b.out_edges {
        nodes_out_edges.push(out_edge.clone());
      }
      if num_out_edges != 0 {
        let start = nodes_out_edges.len() - b.out_edges.len();
        let end = start + num_out_edges;
        for i in start..end {
          inserted_out_edges.push(nodes_out_edges[i].clone());
        }
      }

      let node_def =
        NodeDef::new(b.id, inserted_in_edges, inserted_out_edges, b.priority);
      nodes_defs.push(node_def);
    }

    (nodes_in_edges, nodes_out_edges, nodes_defs)
  }

  // Erases edge from `from` node to `to` node if it exists and it has a weaker
  // ordering than the given `kind`. We rely on the fact that out and in-edges
  // are sorted and use binary search on a critical path.
  fn erase_edge(
    from: &mut NodeDefBuilder,
    to: &mut NodeDefBuilder,
    kind: NodeEdgeKind) -> i64
  {
    debug_assert!(from.id != to.id, "Nodes must be different");
    debug_assert!(from.id < to.id, "Nodes must be ordered");

    // Short-circuit if out or in-edges are empty.
    if from.out_edges.is_empty() || to.in_edges.is_empty() {
      let mut out_edge_count = 0;
      for edge in &from.out_edges {
        if edge_predicate(to.id, edge) { out_edge_count += 1; }
      }
      assert!(out_edge_count == 0, "Unexpected out edge from {:?} to {:?}", from.id, to.id);
      let mut in_edge_count = 0;
      for edge in &to.in_edges {
        if edge_predicate(from.id, edge) { in_edge_count += 1; }
      }
      assert!(in_edge_count == 0, "Unexpected in edge from {:?} to {:?}", from.id, to.id);
      return 0;
    }

    // Short-circuit if out-edges or in-edges don't intersect with `to` or `from`
    // node ids (remember that edges are sorted).
    if from.out_edges.last().unwrap().id < to.id ||
      to.in_edges.first().unwrap().id > from.id
    {
      let mut out_edge_count = 0;
      for edge in &from.out_edges {
        if edge_predicate(to.id, edge) { out_edge_count += 1; }
      }
      assert!(out_edge_count == 0, "Unexpected out edge from {:?} to {:?}", from.id, to.id);
      let mut in_edge_count = 0;
      for edge in &to.in_edges {
        if edge_predicate(from.id, edge) { in_edge_count += 1; }
      }
      assert!(in_edge_count == 0, "Unexpected in edge from {:?} to {:?}", from.id, to.id);
      return 0;
    }

    // Comparator to find a node edge with a given node id.
    //let less_than =
      //|edge: &NodeEdge, id: i64| -> bool {
      //edge.id < id
    //};

    // Check if `from` node has an out edge to `to` node.
    let mut out_edges_index = 0;
    let mut has_out_edge = false;
    for out_edge in &from.out_edges {
      if out_edge.id == to.id { has_out_edge = true; break; }
      out_edges_index += 1;
    }

    // Short-circuit if there is no out edge from `from` node to `to` node.
    if !has_out_edge {
      let mut in_edge_count = 0;
      for edge in &to.in_edges {
        if edge_predicate(from.id, edge) { in_edge_count += 1; }
      }
      assert!(in_edge_count == 0);
      return 0;
    }

    // Check if `to` node has an in edge from `from` node.
    let mut in_edges_index = 0;
    let mut has_in_edge = false;
    for in_edge in &to.in_edges {
      if in_edge.id == from.id { has_in_edge = true; break; }
      in_edges_index += 1;
    }
    debug_assert!(has_in_edge, "In-edge must exist if out-edge exists");
    debug_assert!(to.in_edges[in_edges_index].kind == from.out_edges[out_edges_index].kind,
      "Edges kind must match");

    // At this point we must have exactly one edge between `from` and `to` nodes.
    let mut out_edges_count = 0;
    for edge in &from.out_edges {
      if edge_predicate(to.id, edge) { out_edges_count += 1; }
    }
    assert!(out_edges_count == 1);
    let mut in_edges_count = 0;
    for edge in &to.in_edges {
      if edge_predicate(from.id, edge) { in_edges_count += 1; }
    }
    assert!(in_edges_count == 1);

    // We can't erase an edge with a stronger ordering guarantee.
    if to.in_edges[in_edges_index].kind == NodeEdgeKind::Execution &&
      kind == NodeEdgeKind::Scheduling
    {
      return 0;
    }

    // We erased exactly one edge between `from` and `to` nodes.
    from.out_edges.remove(out_edges_index);
    to.in_edges.remove(in_edges_index);
    return 1;
  }
}

// A base class for an operation that can be executed by the runtime.
pub struct Operation {
  named_nested_operations: Vec<(String, Vec<Operation>)>,
  buffers: Vec<BufferUse>,
  resources: Vec<ResourceUse>
}

impl Operation {
  pub fn default() -> Self {
    Operation {
      named_nested_operations: Vec::new(),
      buffers: Vec::new(),
      resources: Vec::new()
    }
  }

  pub fn new_for_test(buffers: Vec<BufferUse>, resources: Vec<ResourceUse>) -> Self {
    Operation {
      named_nested_operations: Vec::new(),
      buffers: buffers,
      resources: resources,
    }
  }

  pub fn name(&self) -> String {
    "".to_string()
  }

  pub fn op_type_id(&self) -> i64 {
    unimplemented!()
  }

  pub fn buffer_uses(&self) -> Vec<BufferUse> {
    let mut buffers = vec![];
    buffers.clone_from(&self.buffers);
    buffers
  }

  pub fn resource_uses(&self) -> Vec<ResourceUse> {
    let mut resources = vec![];
    resources.clone_from(&self.resources);
    resources
  }

  pub fn to_string(&self) -> String {
    "".to_string()
  }
}

// Edge kind defines execution ordering between two operations. Scheduling
// edge is weaker than an execution edge, as it gives more flexibility
// to the backend runtime to execute operations concurrently.
#[derive(Debug, Clone, PartialEq)]
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
  pub fn new(kind: NodeEdgeKind, id: i64) -> Self {
    NodeEdge { kind, id }
  }

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
#[derive(Debug)]
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
#[derive(Debug, Clone)]
pub struct NodeDefBuilder {
  id: i64,
  priority: i64,
  in_edges: Vec<NodeEdge>,
  out_edges: Vec<NodeEdge>,
}

impl NodeDefBuilder {
  pub fn default() -> Self {
    NodeDefBuilder {
      id: 0, priority: 0, in_edges: Vec::new(), out_edges: Vec::new()
    }
  }
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

// A state of a DFS traversal for transitive reduction.
struct TransitiveResuctionDfsState {
  stack: Vec<NodeEdge>,
  visited: Vec<bool>,
  num_execution_edges: usize,
  num_scheduling_edges: usize,
}

impl TransitiveResuctionDfsState {
  pub fn default() -> Self {
    TransitiveResuctionDfsState {
      stack: Vec::new(),
      visited: Vec::new(),
      num_execution_edges: 0,
      num_scheduling_edges: 0
    }
  }

  pub fn push_to_stack(&mut self, edge: NodeEdge) {
    if !self.visited[edge.id as usize] {
      if edge.kind == NodeEdgeKind::Execution {
        self.num_execution_edges += 1;
      } else {
        self.num_scheduling_edges += 1;
      }
      self.visited[edge.id as usize] = true;
      self.stack.push(edge);
    }
  }

  pub fn push_to_stack_edges(&mut self, edges: Vec<NodeEdge>) {
    for edge in edges {
      self.push_to_stack(edge);
    }
  }

  pub fn pop_from_stack(&mut self) -> NodeEdge {
    let edge = self.stack.last().unwrap();
    if edge.kind == NodeEdgeKind::Execution {
      self.num_execution_edges -= 1;
    } else {
      self.num_scheduling_edges -= 1;
    }
    self.stack.pop().unwrap()
  }

  pub fn empty(&self) -> bool {
    self.stack.is_empty()
  }

  pub fn visited(&mut self, id: i64) {
    self.visited[id as usize] = true;
  }

  pub fn num_visited(&self) -> i64 {
    let mut num = 0;
    for visited in &self.visited {
      if *visited == true { num += 1; }
    }
    num
  }

  pub fn clear(&mut self, num_nodes: usize) {
    self.stack.clear();
    if self.visited.is_empty() {
      self.visited.resize(num_nodes, false);
      return;
    }
    for i in 0..num_nodes {
      self.visited[i] = false;
    }
  }

  pub fn num_execution_edges(&self) -> bool {
    self.num_execution_edges != 0
  }

  pub fn num_scheduling_edges(&self) -> bool {
    self.num_scheduling_edges != 0
  }
}

#[cfg(test)]
mod tests {

  use common::{
    shape_util::ShapeUtil,
    blitz_data::PrimitiveType
  };
  use service::buffer_assignment::{BufferAllocation, BufferAllocationSlice};
  use crate::resource_use::Resource;

use super::*;

  #[test]
  fn test_edge_dependency_ordering() {
    let alloc = BufferAllocation::new(0, 80, 0);

    let slice_shape = ShapeUtil::make_shape(
      &PrimitiveType::F32, vec![10]);
    let slice_0 =
      BufferAllocationSlice::new(alloc.clone(), 0, 40);
    let slice_1 =
      BufferAllocationSlice::new(alloc.clone(), 40, 40);
    let slice_2 =
      BufferAllocationSlice::new(alloc.clone(), 20, 40);

    let mut operations = vec![];
    operations.push(Operation::new_for_test(vec![
      BufferUse::read(slice_0.clone(), slice_shape.clone()),
      BufferUse::write(slice_0.clone(), slice_shape.clone())],
      vec![]));
    operations.push(Operation::new_for_test(vec![
      BufferUse::read(slice_1.clone(), slice_shape.clone()),
      BufferUse::write(slice_1.clone(), slice_shape.clone())],
      vec![]));
    operations.push(Operation::new_for_test(vec![
      BufferUse::read(slice_2.clone(), slice_shape.clone()),
      BufferUse::write(slice_2.clone(), slice_shape.clone())],
      vec![]));

    let execution_graph_wrapper =
      ExecutionGraph::new_from_operations(&operations);
    assert!(execution_graph_wrapper.is_ok());

    let execution_graph = execution_graph_wrapper.unwrap();
    assert!(!execution_graph.is_sequential());
    assert_eq!(execution_graph.source(), &vec![0, 1]);
    assert_eq!(execution_graph.sink(), &vec![2]);

    assert_eq!(execution_graph.out_edges(0),
      &vec![NodeEdge::new(NodeEdgeKind::Execution, 2)]);
    assert_eq!(execution_graph.out_edges(1),
      &vec![NodeEdge::new(NodeEdgeKind::Execution, 2)]);
    assert_eq!(execution_graph.in_edges(2),
      &vec![NodeEdge::new(NodeEdgeKind::Execution, 0), NodeEdge::new(NodeEdgeKind::Execution, 1)]);

    assert_eq!(execution_graph.priority(0), 1);
    assert_eq!(execution_graph.priority(1), 1);
    assert_eq!(execution_graph.priority(2), 0);
  }

  #[test]
  fn test_sequential_ordering() {
    let alloc = BufferAllocation::new(0, 80, 0);
    let slice_shape = ShapeUtil::make_shape(
      &PrimitiveType::F32, vec![10]);
    let slice =
      BufferAllocationSlice::new(alloc.clone(), 0, 40);

    let mut operations = vec![];
    operations.push(Operation::new_for_test(vec![
      BufferUse::read(slice.clone(), slice_shape.clone()),
      BufferUse::write(slice.clone(), slice_shape.clone())],
      vec![]));
    operations.push(Operation::new_for_test(vec![
      BufferUse::read(slice.clone(), slice_shape.clone()),
      BufferUse::write(slice.clone(), slice_shape.clone())],
      vec![]));
    operations.push(Operation::new_for_test(vec![
      BufferUse::read(slice.clone(), slice_shape.clone()),
      BufferUse::write(slice.clone(), slice_shape.clone())],
      vec![]));

    let execution_graph_wrapper =
      ExecutionGraph::new_from_operations(&operations);
    assert!(execution_graph_wrapper.is_ok());

    let execution_graph = execution_graph_wrapper.unwrap();
    assert!(execution_graph.is_sequential());
    assert_eq!(execution_graph.source(), &vec![0]);
    assert_eq!(execution_graph.sink(), &vec![2]);

    assert_eq!(execution_graph.out_edges(0),
      &vec![NodeEdge::new(NodeEdgeKind::Execution, 1)]);
    assert_eq!(execution_graph.out_edges(1),
      &vec![NodeEdge::new(NodeEdgeKind::Execution, 2)]);
    assert_eq!(execution_graph.in_edges(1),
      &vec![NodeEdge::new(NodeEdgeKind::Execution, 0)]);
    assert_eq!(execution_graph.in_edges(2),
      &vec![NodeEdge::new(NodeEdgeKind::Execution, 1)]);

    assert_eq!(execution_graph.priority(0), 2);
    assert_eq!(execution_graph.priority(1), 1);
    assert_eq!(execution_graph.priority(2), 0);
  }

  #[test]
  fn test_token_resource_ordering() {
    let alloc = BufferAllocation::new(0, 80, 0);
    let slice_shape = ShapeUtil::make_shape(
      &PrimitiveType::F32, vec![10]);
    let slice_0 = 
      BufferAllocationSlice::new(alloc.clone(), 0, 40);
    let slice_1 =
      BufferAllocationSlice::new(alloc.clone(), 40, 40);
    let resource = Resource::new(ResourceKind::Token);

    let mut operations = vec![];
    operations.push(Operation::new_for_test(vec![
      BufferUse::read(slice_0.clone(), slice_shape.clone()),
      BufferUse::write(slice_0.clone(), slice_shape.clone())
    ], vec![ResourceUse::write(resource.clone())]));
    operations.push(Operation::new_for_test(vec![
      BufferUse::read(slice_1.clone(), slice_shape.clone()),
      BufferUse::write(slice_1.clone(), slice_shape.clone())
    ], vec![ResourceUse::write(resource.clone())]));

    let execution_graph_wrapper =
      ExecutionGraph::new_from_operations(&operations);
    assert!(execution_graph_wrapper.is_ok());

    let execution_graph = execution_graph_wrapper.unwrap();
    assert_eq!(execution_graph.is_sequential(), true);
    assert_eq!(execution_graph.source(), &vec![0]);
    assert_eq!(execution_graph.sink(), &vec![1]);

    assert_eq!(execution_graph.out_edges(0), 
      &vec![NodeEdge::new(NodeEdgeKind::Execution, 1)]);
    assert_eq!(execution_graph.in_edges(1), 
      &vec![NodeEdge::new(NodeEdgeKind::Execution, 0)]);

    assert_eq!(execution_graph.priority(0), 1);
    assert_eq!(execution_graph.priority(1), 0);
  }

  #[test]
  fn test_collectives_resource_ordering() {
    let alloc = BufferAllocation::new(0, 80, 0);
    let slice_shape = ShapeUtil::make_shape(
      &PrimitiveType::F32, vec![10]);
    let slice_0 = 
      BufferAllocationSlice::new(alloc.clone(), 0, 40);
    let slice_1 =
      BufferAllocationSlice::new(alloc.clone(), 40, 40);
    let resource = Resource::new(ResourceKind::CollectiveComunicator);

    let mut operations = vec![];
    operations.push(Operation::new_for_test(vec![
      BufferUse::read(slice_0.clone(), slice_shape.clone()),
      BufferUse::write(slice_0.clone(), slice_shape.clone())
    ], vec![ResourceUse::write(resource.clone())]));
    operations.push(Operation::new_for_test(vec![
      BufferUse::read(slice_1.clone(), slice_shape.clone()),
      BufferUse::write(slice_1.clone(), slice_shape.clone())
    ], vec![ResourceUse::write(resource.clone())]));
    operations.push(Operation::new_for_test(vec![
      BufferUse::read(slice_1.clone(), slice_shape.clone()),
      BufferUse::write(slice_1.clone(), slice_shape.clone())
    ], vec![ResourceUse::write(resource.clone())]));

    let execution_graph_wrapper =
      ExecutionGraph::new_from_operations(&operations);
    assert!(execution_graph_wrapper.is_ok());

    let execution_graph = execution_graph_wrapper.unwrap();
    assert_eq!(execution_graph.is_sequential(), true);
    assert_eq!(execution_graph.source(), &vec![0]);
    assert_eq!(execution_graph.sink(), &vec![2]);

    assert_eq!(execution_graph.out_edges(0), 
      &vec![NodeEdge::new(NodeEdgeKind::Scheduling, 1)]);
    assert_eq!(execution_graph.in_edges(1), 
      &vec![NodeEdge::new(NodeEdgeKind::Scheduling, 0)]);

    // We have buffer conflicts, and a resource conflict, so in this case we
    // must add an execution edge as it provides stronger ordering guarantee
    assert_eq!(execution_graph.out_edges(1), 
      &vec![NodeEdge::new(NodeEdgeKind::Execution, 2)]);
    assert_eq!(execution_graph.in_edges(2), 
      &vec![NodeEdge::new(NodeEdgeKind::Execution, 1)]);
    
    assert_eq!(execution_graph.priority(0), 2);
    assert_eq!(execution_graph.priority(1), 1);
    assert_eq!(execution_graph.priority(2), 0);
  }

  #[test]
  fn test_transitive_reduction() {
    let alloc = BufferAllocation::new(0, 80, 0);
    let slice_shape = ShapeUtil::make_shape(
      &PrimitiveType::F32, vec![10]);
    let slice = 
      BufferAllocationSlice::new(alloc.clone(), 0, 40);

    let mut operations = vec![];
    operations.push(Operation::new_for_test(vec![
      BufferUse::read(slice.clone(), slice_shape.clone()),
      BufferUse::write(slice.clone(), slice_shape.clone())
    ], vec![]));
    operations.push(Operation::new_for_test(vec![
      BufferUse::read(slice.clone(), slice_shape.clone()),
      BufferUse::write(slice.clone(), slice_shape.clone())
    ], vec![]));
    operations.push(Operation::new_for_test(vec![
      BufferUse::read(slice.clone(), slice_shape.clone()),
      BufferUse::write(slice.clone(), slice_shape.clone())
    ], vec![]));

    let execution_graph_wrapper =
      ExecutionGraph::new_from_operations(&operations);
    assert!(execution_graph_wrapper.is_ok());

    let execution_graph = execution_graph_wrapper.unwrap();
    assert_eq!(execution_graph.source(), &vec![0]);
    assert_eq!(execution_graph.sink(), &vec![2]);

    assert_eq!(execution_graph.out_edges(0), 
      &vec![NodeEdge::new(NodeEdgeKind::Execution, 1)]);
    assert_eq!(execution_graph.in_edges(1), 
      &vec![NodeEdge::new(NodeEdgeKind::Execution, 0)]);
    assert_eq!(execution_graph.out_edges(1), 
      &vec![NodeEdge::new(NodeEdgeKind::Execution, 2)]);
    assert_eq!(execution_graph.in_edges(2), 
      &vec![NodeEdge::new(NodeEdgeKind::Execution, 1)]);
    
    assert_eq!(execution_graph.priority(0), 2);
    assert_eq!(execution_graph.priority(1), 1);
    assert_eq!(execution_graph.priority(2), 0);
  }

  #[test]
  fn test_transitive_reduction_keep_execution_edge() {
    let alloc = BufferAllocation::new(0, 80, 0);
    let slice_shape = ShapeUtil::make_shape(
      &PrimitiveType::F32, vec![10]);
    let slice =
      BufferAllocationSlice::new(alloc, 0, 40);

    let resource = Resource::new(ResourceKind::CollectiveComunicator);
    let mut operations = vec![];

    // All three operations connected with scheduling edges, but because execution
    // edge provides stronger ordering guarantee, we must keep an 0-2 execution
    // edge, or we might get a data race.
    operations.push(Operation::new_for_test(
      vec![BufferUse::write(slice.clone(), slice_shape.clone())],
      vec![ResourceUse::write(resource.clone())]));
    operations.push(Operation::new_for_test(
      vec![],
      vec![ResourceUse::write(resource.clone())]));
    operations.push(Operation::new_for_test(
      vec![BufferUse::write(slice.clone(), slice_shape.clone())],
      vec![ResourceUse::write(resource.clone())]));
    
    let execution_graph_wrapper =
      ExecutionGraph::new_from_operations(&operations);
    assert!(execution_graph_wrapper.is_ok());

    let execution_graph = execution_graph_wrapper.unwrap();
    assert_eq!(execution_graph.source(), &vec![0]);
    assert_eq!(execution_graph.sink(), &vec![2]);
    
    assert_eq!(execution_graph.out_edges(0),
      &vec![NodeEdge::new(NodeEdgeKind::Scheduling, 1), NodeEdge::new(NodeEdgeKind::Execution, 2)]);
    assert_eq!(execution_graph.in_edges(1),
      &vec![NodeEdge::new(NodeEdgeKind::Scheduling, 0)]);
    assert_eq!(execution_graph.in_edges(2),
      &vec![NodeEdge::new(NodeEdgeKind::Execution, 0), NodeEdge::new(NodeEdgeKind::Scheduling, 1)]);

    assert_eq!(execution_graph.priority(0), 2);
    assert_eq!(execution_graph.priority(1), 1);
    assert_eq!(execution_graph.priority(2), 0);
  }
}