#![allow(dead_code)]

// A class describing the source(s) of the Buffer(s) contained in the output of
// a particular HLO instruction. The structure of PointsToSet mirrors the
// structure of the instruction's shape, which may be an arbitrary tree (eg, a
// nested tuple). Each node in this tree corresponds to a single buffer in the
// instruction's output and contains the set of Buffers which might define
// the corresponding buffer.
pub struct PointsToSet {

}
