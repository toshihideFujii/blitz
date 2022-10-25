
#![allow(dead_code)]
/*
An implementation of the RecordVisitor which generates a mapping
between a thread and a range of records representing a block.
*/

// The BlockIndexer will gather all related records associated with a
// process+thread and group them by 'Block'.
struct BlockIndexer {}