#![allow(dead_code)]

use std::collections::HashMap;

use common::{blitz_data::HeapSimulatorTrace, shape::Shape, util::human_readable_num_bytes};
use hlo::{analysis::{alias_info::AliasInfo, hlo_alias_analysis::HloAliasAnalysis,
  hlo_dataflow_analysis::HloDataflowAnalysis, hlo_ordering::HloOrdering},
  hlo_buffer::HloBuffer, hlo_instruction::HloInstruction, hlo_module::HloModule,
  hlo_schdule::HloSchedule, hlo_value::HloValue
};

// This class abstracts an allocation of contiguous memory which can hold the
// values described by LogicalBuffers. Each LogicalBuffer occupies a sub-range
// of the allocation, represented by a Slice. A single BufferAllocation may hold
// LogicalBuffers with disjoint liveness, which may have overlapping Slices. A
// single BufferAllocation may also hold LogicalBuffers with overlapping
// liveness, which must have disjoint Slices.
//
// The abstraction includes information required by the backends for allocation,
// use, and deallocation of the buffer. This includes the LogicalBuffers which
// are held in this allocation through the execution of the computation.
#[derive(Debug, Clone, PartialEq)]
pub struct BufferAllocation {
  index: i64,
  is_thread_local: bool,
  is_tuple: bool,
  is_entry_computation_parameter: bool,
  is_parameter_aliased_with_output: bool,
  is_constant: bool,
  parameter_number: i64,
  param_shape_index: Vec<i64>,
  maybe_live_out: bool,
  size: i64,
  heap_traces: Vec<HeapSimulatorTrace>,
  peak_buffers: Vec<HloValue>,
  assigned_buffers: HashMap<HloValue, OffsetSize>,
  fragmentation_bytes: i64,
}

impl BufferAllocation {
  pub fn new() {
      
  }

  // Returns the index of this allocation.
  pub fn index(&self) -> i64 {
    self.index
  }

  // Whether this allocation is used in a parallel calling context such as
  // inside of a map or reduce computation. Such allocations need to be thread
  // local.
  pub fn is_thread_local(&self) -> bool {
    self.is_thread_local
  }

  pub fn set_is_thread_local(&mut self, is_thread_local: bool) {
    self.is_thread_local = is_thread_local;
  }

  // Whether this allocation can be used by more than one logical buffer.
  pub fn is_reusable(&self) -> bool {
    unimplemented!()
  }

  // Whether this allocation is read-only i.e. backed by memory we cannot write to.
  pub fn is_readonly(&self) -> bool {
    unimplemented!()
  }

  pub fn is_tuple(&self) -> bool {
    self.is_tuple
  }

  pub fn set_is_tuple(&mut self, is_tuple: bool) {
    self.is_tuple = is_tuple;
  }

  // Whether this allocation holds a LogicalBuffer from a parameter of the entry
  // computation. These buffers have lifetimes which may be longer than the
  // Blitz computation.
  pub fn is_entry_computation_parameter(&self) -> bool {
    self.is_entry_computation_parameter
  }

  pub fn is_parameter_aliased_with_output(&self) -> bool {
    self.is_parameter_aliased_with_output
  }

  // Whether this allocation holds a constant.  On the CPU and GPU backends
  // constant allocations are not allocated dynamically, instead we resolve
  // references to these buffer allocations to a global in the readonly section
  // of the binary.
  pub fn is_constant(&self) -> bool {
    self.is_constant
  }

  // If this allocation holds a Buffer from a parameter of the entry
  // computation, this methods returns the parameter number. CHECKs otherwise.
  pub fn parameter_number(&self) -> i64 {
    debug_assert!(self.is_entry_computation_parameter);
    self.parameter_number
  }

  // If this allocation is for a parameter of the entry computation, this
  // function returns which subshape of the parameter the allocation is for.
  pub fn param_shape_index(&self) -> &Vec<i64> {
    debug_assert!(self.is_entry_computation_parameter);
    &self.param_shape_index
  }

  // Returns whether this allocation is assigned a LogicalBuffer which may
  // be live out of the entry computation.
  pub fn maybe_live_out(&self) -> bool {
    self.maybe_live_out
  }

  pub fn set_maybe_live_out(&mut self, maybe_live_out: bool) {
    self.maybe_live_out = maybe_live_out;
  }

  // Returns the size of the allocation. Necessarily this must be at least as
  // large as any LogicalBuffer assigned to this allocation.
  pub fn size(&self) -> i64 {
    self.size
  }

  // Returns the color of the allocation. Only logical buffers with a matching
  // color can reside in this allocation.
  pub fn color(&self) {
    unimplemented!()
  }

  pub fn set_color(&mut self) {
    unimplemented!()
  }

  pub fn assigned_buffers(&self) -> &HashMap<HloValue, OffsetSize> {
    &self.assigned_buffers
  }

  pub fn get_slice(&self, buffer: &HloValue) -> BufferAllocationSlice {
    let os = self.assigned_buffers.get(buffer);
    if os.is_none() { debug_assert!(false); }
    BufferAllocationSlice::new(self.clone(),
      os.as_ref().unwrap().offset, os.as_ref().unwrap().size)
  }

  pub fn to_string(&self) -> String {
    unimplemented!()
  }

  pub fn to_short_string(&self, _human_readable_size: bool) -> String {
    unimplemented!()
  }

  pub fn value_to_string(&self) -> String {
    unimplemented!()
  }

  // The function returns memory usage report for the values belonging to the
  // buffer allocation. The values are grouped by their offset in the
  // allocation. The groups are sorted by the max size(Z-A) of the values in the
  // group. Percentile and more_than_k are used to control the number of groups
  // being reported.
  pub fn memory_usage_report(
    &self, _prefix: &String,
    _percentile: f64,
    _more_than_k: i64) -> String
  {
    unimplemented!()    
  }

  // Whether the buffer is a parameter to or live out of the entry computation.
  pub fn is_input_output(&self) -> bool {
    self.is_entry_computation_parameter() || self.maybe_live_out()
  }

  // Whether the buffer is a temporary buffer allocated before
  // Executable::ExecuteOnStream.
  pub fn is_preallocated_temp_buffer(&self) -> bool {
    !self.is_entry_computation_parameter() &&
    !self.maybe_live_out() &&
    !self.is_thread_local() &&
    !self.is_constant()
  }

  // Add a heap trace which was used to assign slices to logical buffers in this
  // allocation. A single BufferAllocation may include multiple heap traces
  // in the case of the temporary block where there is a heap trace per
  // computation.
  pub fn add_heap_trace(&mut self, heap_trace: HeapSimulatorTrace) {
    self.heap_traces.push(heap_trace);
    // TODO
  }

  // Return the set of heap traces used to assign slices to logical buffers in
  // this allocation.
  pub fn heap_traces(&self) -> &Vec<HeapSimulatorTrace> {
    &self.heap_traces
  }

  // Returns the LogicalBuffers which are live at the point of peak memory usage
  // for this allocation. The point of peak memory usage is the point at which
  // the total size of all live logical buffers is maximal. If peak memory is
  // reached at multiple points, the set of logical buffers live at the earliest
  // maximal point is returned. The vector is stably sorted by
  // BufferValue::Index.
  pub fn peak_memory_logical_buffers(&self) -> &Vec<HloValue> {
    &self.peak_buffers
  }

  // Get the number of bytes lost to fragmentation. This is equal to the
  // difference between the size of the allocation and the size of the maximal
  // live set.
  pub fn fragmentation_bytes(&self) -> i64 {
    self.fragmentation_bytes
  }

  pub fn set_entry_computation_parameter(
    &mut self,
    parameter_number: i64,
    param_shape_index: Vec<i64>,
    parameter_aliased_with_output: bool)
  {
    self.is_entry_computation_parameter = true;
    self.is_parameter_aliased_with_output = parameter_aliased_with_output;
    self.parameter_number = parameter_number;
    self.param_shape_index = param_shape_index;
  }

  pub fn set_constant(&mut self, is_constant: bool) {
    self.is_constant = is_constant;
  }

  // Adds a LogicalBuffer to the set assigned to this buffer.
  fn add_assignment(&mut self, _buffer: HloValue, _offset: i64, _size: i64) {
    unimplemented!()
  }

  fn set_index(&mut self, index: i64) {
    self.index = index;
  }

  fn set_size(&mut self, size: i64) {
    self.size = size;
  }
}

// A Slice represents a contiguous portion of a memory allocation. It is used
// to identify the memory range that a LogicalBuffer corresponds to.
#[derive(Debug, PartialEq)]
pub struct BufferAllocationSlice {
  allocation: Option<BufferAllocation>,
  offset: i64,
  size: i64,
}

impl BufferAllocationSlice {
  pub fn default() -> Self {
    BufferAllocationSlice { allocation: None, offset: 0, size: 0 }
  }

  pub fn new(allocation: BufferAllocation, offset: i64, size: i64) -> Self {
    BufferAllocationSlice {
      allocation: Some(allocation),
      offset: offset,
      size: size
    }
  }

  pub fn allocation(&self) -> Option<&BufferAllocation> {
    Some(self.allocation.as_ref().unwrap())
  }

  pub fn index(&self) -> i64 {
    self.allocation().unwrap().index()
  }

  pub fn offset(&self) -> i64 {
    self.offset
  }

  pub fn size(&self) -> i64 {
    self.size
  }

  pub fn overlaps_with(&self, other: &BufferAllocationSlice) -> bool {
    let end = self.offset + self.size;
    let other_end = other.offset + other.size;
    self.index() == other.index() && self.offset < other_end && end > other.offset
  }

  pub fn to_string(&self) -> String {
    let mut out = "{index:".to_string();
    if self.allocation.is_none() {
      out.push_str("-1");
    } else {
      out.push_str(&self.index().to_string());
    }
    out.push_str(", offset:");
    out.push_str(&self.offset.to_string());
    out.push_str(", size:");
    out.push_str(&self.size().to_string());
    out.push_str("}");
    out
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OffsetSize {
  offset: i64,
  size: i64
}

// This class encapsulates an assignment of the LogicalBuffers in an Blitz
// module to a set of BufferAllocations.
pub struct BufferAssignment<'module> {
  allocations: Vec<BufferAllocation>,
  temp_allocation_total_size: i64,
  multiheap_size_constraint_per_heap: i64,
  allocation_index_for_value: HashMap<HloValue,i64>,
  module: &'module HloModule,
  hlo_ordering: HloOrdering<'module>,
  // buffer_size
  // color_alignment
  alias_analysis: HloAliasAnalysis<'module>,
  //hlo_live_range
  stats: Stats,
  cached_buffer_sizes: HashMap<i64, i64>
}

impl<'module> BufferAssignment<'module> {
  // Returns the vector containing all buffer allocations in this assignment.
  pub fn allocations(&self) -> &Vec<BufferAllocation> {
    &self.allocations
  }

  // Returns the total size allocation holding all temporary buffers.
  pub fn temp_allocation_total_size(&self) -> i64 {
    self.temp_allocation_total_size
  }

  pub fn multiheap_size_constraint_per_heap(&self) -> i64 {
    self.multiheap_size_constraint_per_heap
  }

  // Returns whether the given buffer has been assigned an allocation.
  pub fn has_allocation(&self, value: &HloValue) -> bool {
    self.allocation_index_for_value.contains_key(value)
  }

  // Returns whether the given (logical) buffer with the id has been assigned an
  // allocation.
  pub fn has_allocation_by_id(&self, value_id: i64) -> bool {
    self.has_allocation(self.dataflow_analysis().get_value(value_id))
  }

  pub fn has_allocation_by_buffer(&self, buffer: &HloBuffer) -> bool {
    self.allocation_index_for_value.contains_key(&buffer.values()[0])
  }

  // Returns the allocation that a particular LogicalBuffer has been assigned
  // to. CHECKs if buffer has not been assigned an allocation.
  pub fn get_assigned_allocation(&self, value: &HloValue) -> &BufferAllocation {
    debug_assert!(self.has_allocation(value));
    let index = *self.allocation_index_for_value.get(value).unwrap();
    self.get_allocation(index)
  }

  pub fn get_assigned_allocation_by_buffer(&self, buffer: &HloBuffer) -> &BufferAllocation {
    self.get_assigned_allocation(&buffer.values()[0])
  }

  // Returns the allocation with the given index. CHECKs if no allocation exists
  // with the given index.
  pub fn get_allocation(&self, index: i64) -> &BufferAllocation {
    debug_assert!(index >= 0);
    debug_assert!((index as usize) < self.allocations.len());
    &self.allocations[index as usize]
  }

  // Returns the allocation with the given instruction and shape index. nullptr
  // if no allocation exists.
  pub fn get_instruction_allocation(
    &self, hlo: &HloInstruction, shape_index: &Vec<i64>) -> Option<&BufferAllocation>
  {
    let value = &self.dataflow_analysis()
      .get_value_set(hlo, shape_index).values()[0];

    if !self.has_allocation(value) { return None; }
    Some(self.get_assigned_allocation(value))
  }

  // Builds and returns a vector containing the slices which might contain the
  // subvalue at the given index of given instruction.
  pub fn get_all_slices(
    &self,
    _instruction: &HloInstruction,
    _index: &Vec<i64>) -> &Vec<BufferAllocationSlice>
  {
    unimplemented!()    
  }

  // Convenience function which returns whether the buffer of the
  // instruction at the given index is assigned an allocation.
  pub fn has_allocation_at(
    &self, instruction: &HloInstruction, index: &Vec<i64>) -> bool
  {
    let values = self.dataflow_analysis()
      .get_value_set(instruction, index).values();
    for key in values {
      if self.allocation_index_for_value.contains_key(key) {
        return true;
      }
    }
    false
  }

  // Convenience function which returns whether the top-level buffer of the
  // instruction (index == {}) is assigned an allocation.
  pub fn has_top_level_allocation(&self, instruction: &HloInstruction) -> bool {
    self.has_allocation_at(instruction, &vec![])
  }

  // Convenience function which returns the unique slice containing the buffer
  // at the given index of the given instruction. If a slice is not assigned or
  // the slice cannot be determined at compile time then an error is returned.
  pub fn get_unique_slice(
    &self,
    instruction: &HloInstruction,
    index: &Vec<i64>) -> Result<BufferAllocationSlice, String>
  {
    println!("Trying to find unique slice for {:?} [{:?}]",
      instruction.name(), index);
    let mut result = BufferAllocationSlice::default();
    let values = self.dataflow_analysis()
      .get_value_set(instruction, index).values();
    for value in values {
      println!("Examining value {:?}", value);
      if self.has_allocation(value) {
        println!("Has allocation");
        let slice =
          self.get_assigned_allocation(value).get_slice(value);
        if result.allocation().is_none() {
          result = slice;
        } else if result != slice {
          panic!("BufferAllocationSlice for instruction {:?} at index {:?} cannot 
            be determined at compile-time.", instruction.name(), index);
        }
      } else {
        println!("No allocation");
      }
    }
    if result.allocation().is_none() {
      let mut err_msg =
        "BufferAllocationSlice not assigned for instruction ".to_string();
      err_msg.push_str(&instruction.name());
      panic!("BufferAllocationSlice not assigned for instruction {:?} as index {:?}",
        instruction.name(), index);
    }
    Ok(result)
  }

  // Like GetUniqueSlice but fixes the index to the top-level of the shape
  // (index = {}).
  pub fn get_unique_top_level_slice(
    &self,
    instruction: &HloInstruction) -> Result<BufferAllocationSlice, String>
  {
    self.get_unique_slice(instruction, &vec![])
  }

  // Like GetUniqueTopLevelSlice but returns the slice for the output of the
  // entry computation of the HLO module (ie, the result of the XLA
  // computation).
  pub fn get_unique_top_level_output_slice(
    &self) -> Result<BufferAllocationSlice, String>
  {
    self.get_unique_top_level_slice(
      self.module.entry_computation().as_ref().unwrap().root_instruction())
  }

  // Returns the set BufferValues which may be the source of the value at the
  // given index and instruction.
  pub fn get_source_buffers(
    &self, _instruction: &HloInstruction, _index: &Vec<i64>) -> &Vec<HloValue>
  {
    unimplemented!()  
  }

  // Returns true if 'hlo_a{shape_index_a}' and 'hlo_b{shape_index_b}'
  // share the same BufferAllocation::Slice.
  // Returns false otherwise.
  // REQUIRES: BufferAssignment assigned allocations to both instructions.
  pub fn shares_slice_at_index(
    &self,
    hlo_a: &HloInstruction,
    shape_index_a: &Vec<i64>,
    hlo_b: &HloInstruction,
    shape_index_b: &Vec<i64>) -> bool
  {
    self.get_unique_slice(hlo_a, shape_index_a) ==
    self.get_unique_slice(hlo_b, shape_index_b)
  }

  // Returns true if the top-level buffers of hlo_a and hlo_b are the same.
  // REQUIRES: HasTopLevelAllocation(hlo_a) && HasTopLevelAllocation(hlo_b).
  pub fn shares_top_level_slice(
    &self, hlo_a: &HloInstruction, hlo_b: &HloInstruction) -> bool
  {
    self.shares_slice_at_index(
      hlo_a, &vec![], hlo_b, &vec![])    
  }

  // Returns true if hlo_a and hlo_b both have at least one buffer assigned for
  // their top-level and each of their nested shape indices, and if hlo_a's
  // buffers are all different from hlo_b's buffers.
  pub fn have_disjoint_slices(
    &self, _hloa: &HloInstruction, _hlo_b: &HloInstruction) -> bool
  {
    unimplemented!()  
  }

  pub fn dataflow_analysis(&self) -> &HloDataflowAnalysis {
    self.alias_analysis.dataflow_analysis()    
  }

  pub fn alias_analysis(&self) -> &HloAliasAnalysis {
    &self.alias_analysis
  }

  pub fn hlo_ordering(&self) -> &HloOrdering {
    &self.hlo_ordering
  }

  // Returns the HloLiveRange object used to construct this assignment.
  pub fn hlo_live_range(&self) {
    unimplemented!()
  }

  // Is in use by many compilers to dump the buffer-assignment info.
  pub fn to_string(&self) -> String {
    unimplemented!()
  }

  // Returns a memory usage report with the list of buffer allocations ordered
  // by the size(Z-A) and the values assigned to each buffer allocation.
  pub fn memory_usage_report(&self, percentile: f64, more_than_k: i64) -> String {
    let mut output = "".to_string();
    let mut total_size = 0;
    for allocation in self.allocations() {
      total_size += allocation.size();
    }
    output.push_str("Total bytes used: ");
    output.push_str(&total_size.to_string());
    output.push_str("\nAllocations sorted by size:˜\n\n");
    //self.allocations.sort();

    let cumulative_size = 0;
    output.push_str("cumulative_size: total_size - cumulative_size; allocation\n");

    output.push_str("-------------------------\n");
    let mut index = 0;
    for allocation in self.allocations() {
      output.push_str(&cumulative_size.to_string());
      output.push_str("(");
      let value = 100.0 * (cumulative_size / total_size) as f64;
      output.push_str(&value.to_string());
      output.push_str("); ");
      output.push_str(&(total_size - cumulative_size).to_string());
      output.push_str("; ");
      output.push_str(&allocation.to_short_string(true));

      // Skip the rest of the allocations if they are less than percentile of the
      // total size and not more than k.
      index += 1;
      if index > more_than_k &&
        (total_size - cumulative_size) < total_size * (percentile as i64)
      {

        break;
      }
    }

    output.push_str("\n\nAllocations sorted by size with their values:\n");
    for allocation in self.allocations() {
      if allocation.assigned_buffers().len() == 1 {
        output.push_str(&allocation.to_short_string(true));
      } else {
        output.push_str(&allocation.to_short_string(true));
        output.push_str("\n");
        output.push_str(&allocation.memory_usage_report(
          &"\t".to_string(), percentile, more_than_k));
      }
    }
    output
  }

  // Verbose string tailored to debugging OOMs, includes the Hlo op metadata for
  // every buffer associated with each allocation.
  pub fn to_verbose_string(
    &self, _alias_info: &AliasInfo, _max_buffers_to_show: usize) -> String
  {
    unimplemented!()    
  }

  // Is in use by tpu compiler to dump the buffer info.
  pub fn buffer_info_string(&self) -> String {
    unimplemented!()
  }

  // Returns string representation of buffer assignment statistics. Also
  // calculates and returns the total fragmentation.
  pub fn stats_string(&self, alias_info: &AliasInfo) -> String {    
    let mut s = String::new();
    s.push_str("BufferAssignment stats:\n");
    s.push_str("     parameter allocation: ");
    s.push_str(&human_readable_num_bytes(self.stats.parameter_allocation_bytes));
    s.push_str("\n");
    s.push_str("     constant allocation: ");
    s.push_str(&human_readable_num_bytes(self.stats.constant_allocation_bytes));
    s.push_str("\n");
    s.push_str("maybe_live_out allocation: ");
    s.push_str(&human_readable_num_bytes(self.stats.maybe_live_out_allocation_bytes));
    s.push_str("\n");
    s.push_str("preallocated temp allocation: ");
    s.push_str(&human_readable_num_bytes(self.stats.preallocated_temp_allocation_bytes));
    s.push_str("\n");

    if self.stats.preallocated_temp_fragmentation_bytes >= 0 {
      let percent = 100.0 *
        (self.stats.preallocated_temp_fragmentation_bytes as f64) /
        (self.stats.preallocated_temp_allocation_bytes as f64);
      s.push_str("     preallocated temp fragmentation: ");
      s.push_str(&human_readable_num_bytes(
        self.stats.preallocated_temp_fragmentation_bytes));
      s.push_str("(");
      s.push_str(&percent.to_string());
      s.push_str(")\n");
    }

    s.push_str("     total allocation: ");
    s.push_str(&human_readable_num_bytes(self.stats.total_allocation_bytes));
    let total_fragmentation_bytes =
      self.compute_total_fragmentation_bytes(alias_info);
    if total_fragmentation_bytes.is_ok() &&
      *total_fragmentation_bytes.as_ref().unwrap() >= 0 {
      let percent = 100.0 *
        (*total_fragmentation_bytes.as_ref().unwrap() as f64) /
        (self.stats.total_allocation_bytes as f64);
      s.push_str("     total fragmentation: ");
      s.push_str(&human_readable_num_bytes(total_fragmentation_bytes.unwrap()));
      s.push_str("(");
      s.push_str(&percent.to_string());
      s.push_str(")");
    }
    s
  }

  pub fn get_stats(&self) -> &Stats {
    &self.stats
  }

  fn new_empty_allocation(&self, _size: i64, _color: i64) -> BufferAllocation {
    //let index = self.allocations.len();
    unimplemented!()
  }

  // Helper that calls NewEmptyAllocation and AddAssignment in one call,
  // creating an allocation containing a single LogicalBuffer.
  fn new_allocation(
    &mut self, buffer: &HloBuffer, size: i64) -> Result<BufferAllocation, String>
  {
    let color = buffer.color();
    let mut allocation = self.new_empty_allocation(size, color);
    let result = self.add_assignment_by_buffer(
      &mut allocation, buffer, 0, size);
    check_error(&result);
    Ok(allocation)
  }

  fn add_assignment(
    &self,
    allocation: &mut BufferAllocation,
    value: &HloValue,
    offset: i64,
    size: i64) -> Result<(), String>
  {
    allocation.add_assignment(value.clone(), offset, size);
    if self.alias_analysis().value_lives_out(value) {
      println!("HloValue lives out: {:?}", value.to_string());
      println!("Set maybe live out: {:?}", allocation.to_string());
      allocation.set_maybe_live_out(true);
    }
    Ok(())
  }

  fn add_assignment_by_buffer(
    &mut self,
    allocation: &mut BufferAllocation,
    buffer: &HloBuffer,
    offset: i64,
    size: i64) -> Result<(), String>
  {
    debug_assert!(allocation.is_reusable() || allocation.assigned_buffers.is_empty());
    for buf_value in buffer.values() {
      debug_assert!(!self.allocation_index_for_value.contains_key(buf_value));
      allocation.add_assignment(buf_value.clone(), offset, size);
      self.allocation_index_for_value.insert(buf_value.clone(), allocation.index());
    }
    if self.alias_analysis().buffer_lives_out(buffer) {
      println!("HloBuffer lives out: {:?}", buffer.to_string());
      println!("Set maybe live out: {:?}", allocation.to_string());
      allocation.set_maybe_live_out(true);
    }
    Ok(())
  }

  fn module(&self) -> &'module HloModule {
    self.module
  }

  // Mutable accessors for allocations.
  fn get_mutable_assigned_allocation(&self, buffer: &HloBuffer) -> &BufferAllocation {
    self.get_assigned_allocation_by_buffer(buffer)
  }

  fn hlo_buffer_size(&mut self, buffer: &HloBuffer) -> i64 {
    let buf_size = self.cached_buffer_sizes.get(&buffer.id());
    if buf_size.is_some() { return *buf_size.unwrap(); }
    let mut result = 0;
    for value in buffer.values() {
      result = i64::max(result, BufferAssignment::size_function(value)) ;
    }
    self.cached_buffer_sizes.insert(buffer.id(), result);
    result
  }

  fn size_function(_buf_value: &HloValue) -> i64 {
    unimplemented!()
  }

  // Combines allocations of temporary buffers into one big BufferAllocation.
  fn combine_temp_allocations(&self) {
    unimplemented!()
  }

  // Computes stats for the assignment, to be retrieved by GetStats.
  fn compute_summary_stats(&mut self) {
    let mut allocations = vec![];
    allocations.clone_from(self.allocations());
    for allocation in allocations {
      if allocation.is_entry_computation_parameter() {
        self.stats.parameter_allocation_count += 1;
        self.stats.parameter_allocation_bytes += allocation.size();
      }
      if allocation.is_constant() {
        self.stats.constant_allocation_count += 1;
        self.stats.constant_allocation_bytes += allocation.size();
      }
      if allocation.maybe_live_out() {
        self.stats.maybe_live_out_allocation_count += 1;
        self.stats.maybe_live_out_allocation_bytes += allocation.size();
      }
      if allocation.is_preallocated_temp_buffer() {
        self.stats.preallocated_temp_allocation_count += 1;
        self.stats.preallocated_temp_allocation_bytes += allocation.size();
      }
      self.stats.total_allocation_count += 1;
      self.stats.total_allocation_bytes += allocation.size();
    }
  }

  // Calculates and returns the total fragmentation in bytes.
  fn compute_total_fragmentation_bytes(
    &self, _alias_info: &AliasInfo) -> Result<i64, String>
  {
    let mut schedule = HloSchedule::new(self.module);
    let mut schedule_complete = true;
    let callback =
      |_module: &HloModule| -> Result<(Vec<Shape>, Shape), String> {
        Ok((vec![], Shape::new()))
      };
    for computation in self.module.computations(callback) {
      if !computation.is_fusion_computation() {
        let sequence =
          self.hlo_ordering().sequential_order(computation);
        if sequence.is_none() {
          schedule_complete = false;
        } else {
          schedule.set_sequence(computation, sequence.unwrap());
        }
      }
    }
    if schedule_complete {
      let result = schedule.verify();
      check_error(&result);
    }
    Ok(-1)
  }
}

pub struct Stats {
  parameter_allocation_count: i64,
  parameter_allocation_bytes: i64,
  constant_allocation_count: i64,
  constant_allocation_bytes: i64,
  maybe_live_out_allocation_count: i64,
  maybe_live_out_allocation_bytes: i64,
  preallocated_temp_allocation_count: i64,
  preallocated_temp_allocation_bytes: i64,
  preallocated_temp_fragmentation_bytes: i64,
  total_allocation_count: i64,
  total_allocation_bytes: i64,
}

fn check_error<T>(value: &Result<T, String>) {
  if value.is_err() {
    let err_msg = value.as_ref().err().unwrap();
    assert!(false, "{:?}", err_msg);
  }
}