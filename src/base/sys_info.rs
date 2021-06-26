//use rlimit::{getrlimit, Resource};

// Returns the number of logical processors/core on the current machine.
pub fn number_of_processors() -> i64 {
    return 0;
}

// Returns the number of bytes of phisical memory on the current machine.
pub fn amount_of_phisical_memory() -> i64 {
    return 0;
}

// Returns the number of bytes of virtual memory of this process.
// A return value of zero means there is no limit on the available virtual memory.
/*
pub fn amount_of_virtual_memory() -> std::result::Result<(rlimit::Rlim, rlimit::Rlim), std::io::Error> {
    //return 0;
    //getrlimit(Resource::DATA)
}
*/
