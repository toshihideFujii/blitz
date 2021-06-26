// - Object
//   - Smi          (immediate small integer)
//   - HeapObject   (superclass for everything allocated in the heap)
//     - JSReceiver (suitable for property access)
//       - JSObject

pub enum WriteBarrierMode {
    SkipWriteBarrier,
    UnsafeSkipWriteBarrier,
    UpdateWeakWriteBarrier,
    UpdateEphemeronKeyWriteBarrier,
    UpdateWriteBarrier,
}

pub enum PropertyNormalizationMode {
    ClearInobjectProperties,
    KeepInobjectProperties,
}

pub enum TransitionFlag {
    InsertTransition,
    OmitTransition,
}

pub enum SimpleTransitionFlag {
    SimplePropertyTransition,
    PropertyTransition,
    SpecialTransition,
}

// Instance size sentinel for objects of variable size.
pub const VARIABLE_SIZE_SENTINEL: i64 = 0;

pub const STUB_MAJOR_KEY_BITS: i64 = 8;
//pub const STUB_MINOR_KEY_BITS: i64 = SMI_VALUE_SIZE - STUB_MAJOR_KEY_BITS - 1;

pub enum ComparisonResult {
    LessThan,    // x < y
    Equal,       // x = y
    GreaterThan, // x > y
    Undefined,   // at least one of x or y was undefined or NaN
}

pub enum OnNonExistent {
    ThrowReferenceError,
    ReturnUndefined,
}

// The element types selection for CreateListFromArrayLike.
pub enum ElementTypes {
    All,
    StringAndSymbol,
}

pub struct Object {}

pub struct MapWord {}

pub enum EnsureElementsMode {
    DontAllowDoubleElements,
    AllowCopiedDoubleElements,
    AllowConvertedDoubleElements,
}

pub enum AccessorComponent {
    AccessorGetter,
    AccessorSetter,
}

// Utility superclass for stack-allocated objects that must be updated on gc.
// It provides two ways for the gc to update instances, either iterating or updating after gc.
struct Relocatable {}

// Helper class for setting and getting a bit in an integer.
struct BooleanBit {}
