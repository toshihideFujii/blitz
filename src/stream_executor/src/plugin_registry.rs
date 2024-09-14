#![allow(dead_code)]

// Enumeration to list the supported types of plugins / support libraries.
pub enum PluginKind {
  Invalid,
  Blas,
  Dnn,
  Fft,
}

// Containers for the sets of registered factories, by plugin kind.
struct Factories {}

// The PluginRegistry is a singleton that maintains the set of registered
// "support library" plugins. Currently, there are four kinds of plugins:
// BLAS, DNN, and FFT. Each interface is defined in the corresponding
// gpu_{kind}.h header.
//
// At runtime, a StreamExecutor object will query the singleton registry to
// retrieve the plugin kind that StreamExecutor was configured with (refer to
// the StreamExecutor declarations).
//
// Plugin libraries are best registered using REGISTER_MODULE_INITIALIZER,
// but can be registered at any time. When registering a DSO-backed plugin, it
// is usually a good idea to load the DSO at registration time, to prevent
// late-loading from distorting performance/benchmarks as much as possible.
pub struct PluginRegistry {}

impl PluginRegistry {
  pub fn instance() {}

  // Registers the specified factory with the specified platform.
  // Returns a non-successful status if the factory has already been registered
  // with that platform (but execution should be otherwise unaffected).
  pub fn register_factory<T>(
    &self, _platform_id: i64, _name: String, _factory: T) -> Result<(), String>
  {
    unimplemented!()
  }

  // Return true if the factory/kind has been registered for the
  // specified platform and plugin kind and false otherwise.
  pub fn has_factory(&self, _platform_id: i64, _plugin_kind: PluginKind) -> bool {
    unimplemented!()
  }

  // Retrieves the factory registered for the specified kind,
  // or a absl::Status on error.
  pub fn get_factory<T>(&self, _platform_id: i64) -> Result<T, String> {
    unimplemented!()
  }
}