// JSReceiver includes types on which properties can be defined, i.e.,
// JSObject and JSProxy.
struct JSReceiver {}

impl JSReceiver {}

// The JSObject describes real heap allocated JavaScript objects with properties.
// Note that the map of JSObject changes during execution to enable inline caching.
struct JSObject {}

impl JSObject {}
