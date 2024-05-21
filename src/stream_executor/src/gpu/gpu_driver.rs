#![allow(dead_code)]

// Identifies the memory space where an allocation resides.
pub enum MemorySpace {
  Host,
  Device,
}

pub enum GpuDriverEventFlags {
  Default,
  DisableTiming,
}

pub fn memory_space_string() {}

struct GraphInstantiateFlags {
  auto_free_on_launch: bool,
  upload: bool,
  device_launch: bool,
  use_node_priority: bool,
}

// Graph update result.
enum GraphExecuteUpdateResult {
  Success,
  Error,
  TopologyChanged,
  NodeTypeChanged,
  FunctionChanged,
  ParametersChanged,
  NotSupported,
  UnsupportedFunctionChange,
  AttributesChanged,
}

struct GraphExecUpdateResultInfo {}

// Graph node type.
enum GraphNodeType {
  Kernel,
  Memcpy,
  Host,
  Graph,
  Empty,
  WaitEvent,
  EventRecord,
  ExtSemasSignal,
  ExtSemasWait,
  MemAlloc,
  MemFree,
  BatchMemOp,
}

struct GpuGraphConditionalNodeParams {}

// Memory protection flags for mappings.
enum MemAccessFlags {
  None,
  Read,
  ReadWrite,
}

// Specifies the type of memory location.
enum MemLocationType {
  Invalid,
  Device,
  Host,
  HostNuma,
  HostNumaCurrent,
}

// The memory allocation type.
enum MemAllocationType {
  Invalid,
  Pinned,
}

struct ScopedActivateContext {}

pub struct GpuDriver {}

impl GpuDriver {
  pub fn init() {}
  pub fn device_from_context() {}
  pub fn create_stream() {}
  pub fn destroy_stream() {}
  pub fn init_event() {}
  pub fn destroy_event() {}
  pub fn device_allocate() {}
  pub fn device_deallocate() {}
  pub fn unified_memory_allocate() {}
  pub fn unified_memory_deallocate() {}
  pub fn host_allocate() {}
  pub fn host_deallocate() {}
  pub fn host_register() {}
  pub fn host_unregister() {}
  pub fn get_gpu_stream_priority() {}

  pub fn get_device() {}
  pub fn get_device_name() {}
  pub fn create_context() {}
  pub fn destroy_context() {}
  pub fn get_context_handle() {}
  pub fn func_get_attribute() {}
  pub fn func_set_cache_config() {}
  pub fn context_get_shared_mem_config() {}

  pub fn context_set_shared_mem_config() {}
  pub fn launch_kernel() {}
  pub fn create_graph() {}
  pub fn destroy_graph() {}
  pub fn stream_begin_capture() {}
  pub fn stream_begin_capture_to_graph() {}
  pub fn stream_end_capture() {}

  pub fn graph_instantiate() {}
  pub fn graph_launch() {}
  pub fn graph_node_set_enabled() {}

  pub fn graph_exec_update() {}

  pub fn graph_node_get_type() {}
  pub fn destroy_graph_exec() {}
  pub fn graph_debug_dot_print() {}
  pub fn strean_is_capturing() {}
  pub fn device_graph_mem_trim() {}
  pub fn graph_conditional_handle_create() {}

  pub fn graph_add_node() {}
  pub fn graph_add_empty_node() {}
  pub fn graph_add_kernel_node() {}
  pub fn graph_exec_kernel_node_set_params() {}

  pub fn graph_add_mem_alloc_node() {}
  pub fn graph_get_mem_alloc_node_params() {}
  pub fn graph_add_mem_free_node() {}
  pub fn graph_add_memcpy_d2d_node() {}
  pub fn graph_exec_memcpy_d2d_node_set_params() {}
  pub fn graph_add_memset_node() {}
  pub fn graoh_exec_memset_node_set_params() {}
  pub fn graph_add_child_node_set_params() {}
  pub fn load_ptx() {}
  pub fn load_cubin() {}
  pub fn load_hsaco() {}
  pub fn get_module_function() {}
  pub fn get_module_symbol() {}
  pub fn upload_module() {}
  pub fn synchronous_memset_uint8() {}
  pub fn synchronous_memset_uint32() {}
  pub fn asynchronous_memset_uint8() {}
  pub fn asynchronous_memset_uint32() {}
  pub fn synchronous_memcpy_d2h() {}
  pub fn synchronous_memcpy_h2d() {}
  pub fn synchronous_memcpy_d2d() {}

  pub fn add_stream_callback() {}
  pub fn wait_stream_on_event() {}
  pub fn synchronize_stream() {}
  pub fn synchronize_context() {}
  pub fn is_stream_idle() {}
  pub fn can_enable_peer_access() {}
  pub fn enable_peer_access() {}
  pub fn get_event_elapsed_time() {}
  pub fn record_event() {}
  pub fn query_event() {}
  pub fn get_pointer_context() {}
  pub fn get_pointer_device() {}
  pub fn get_pointer_address_range() {}
  pub fn get_compute_capability() {}
  pub fn get_gpu_isa_version() {}
  pub fn get_gpu_gcn_arch_name() {}

  pub fn get_multiprocessor_count() {}
  pub fn get_max_threads_per_multiprocessor() {}
  pub fn get_max_threads_per_block() {}
  pub fn get_max_shared_memory_per_core() {}
  pub fn get_max_shared_memory_per_block() {}
  pub fn get_max_shared_memory_per_block_optin() {}
  pub fn get_max_registers_per_block() {}
  pub fn get_threads_per_warp() {}
  pub fn get_grid_limits() {}
  pub fn get_device_properties() {}
  pub fn get_device_attribute() {}
  pub fn is_ecc_enabled() {}
  pub fn get_device_total_memory() {}
  pub fn get_device_memory_info() {}
  pub fn get_pci_bus_id() {}
  pub fn get_device_count() {}
  pub fn get_driver_version() {}
  pub fn get_max_occupied_blocks_per_core() {}
}