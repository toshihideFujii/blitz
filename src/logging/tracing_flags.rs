

// This struct contains a set of flags that can be modified from
// multiple threads at runtime unlike the normal FLAG_-like flags
// which are not modified after blitz instance is initialized.

pub fn is_runtime_stats_enabled() -> bool {
    return true
}

pub fn is_gc_enabled() -> bool {
    return true
}

pub fn is_gc_stats_enabled() -> bool {
    return true
}

pub fn is_ic_stats_enabled() -> bool {
    return true
}

pub fn is_zone_stats_enabled() -> bool {
    return true
}