

// From http://msdn.microsoft.com/en-us/library/bb204633.aspx,
// the sum of FramesToSkip and FramesToCapture must be less than 63,
// so set it to 62. Even if on POSIX it clould be a larger value,
// it usually doesn't give much more information.
const MAX_TRACES: i32 = 62;

// A stacktrace can be helpful in debugging.
// For example, you can include a stacktrace member in a object
// so that you can see where the given object was created from.
struct StackTrace {
    // The number of valid frames in |trace|.
    count: usize;
}

impl StackTrace {
    // Creates a stacktrace from the current location.
    fn new() -> StackTrace {
        StackTrace{ count:0 }
    }

    // Gets an array of instruction pointer values.
    // |count| will be set to the number of elements in the returned arary.
    fn addresses(count: usize) {

    }

    // Prints the stacktrace to stderr.
    fn print() {

    }

    // Resolves backtrace to symbols and returns as string.
    fn toString() -> String {
        ""
    }
}