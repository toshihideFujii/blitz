

struct MarkingWorklists {}

impl MarkingWorklists {
    pub fn new() {}

    pub fn marking_worklist() {}

    pub fn not_fully_constructed_worklist() {}

    pub fn previously_not_fully_constructed_worklist() {}

    pub fn write_barrier_worklist() {}

    pub fn weak_callback_worklist() {}

    pub fn concurent_marking_bailout_worklist() {}

    pub fn discovered_ephemeron_pairs_worklist() {}

    pub fn ephemeron_pairs_for_processing_worklist() {}

    pub fn weak_containers_worklist() {}

    pub fn retrace_marked_objects_worklist() {}

    fn clear_for_testing() {}
}