// Flags for TurboProp.
// Enable experimental turboprop mid-tier compiler.
pub static FLAG_TURBOPROP: bool = false;

// Enable mid-tier register allocator for turboprop.
pub static FLAG_TURBOPROP_MID_TIER_REG_ALLOC: bool = false;

// Enable experimental turboprop compiler without further tierup to turbofan.
pub static FLAG_TURBOPROP_AS_TOPTIER: bool = false;

// Max number of map checks to perform in mimorphic state.
pub static FLAG_MAX_MINIMORPHIC_MAP_CHECKS: u64 = 4;

// Scale factor for reduction in bytecode that can be inline for.
pub static FLAG_TURBOPROP_INLINE_SCALING_FACTOR: u64 = 4;

// The scale factor determines the interrupt budget when tiering up from
// TurboProp to TurboFan.
pub static FLAG_INTERRUPT_BUDGET_SCALE_FACTOR_FOR_TOP_TIER: u64 = 5;
