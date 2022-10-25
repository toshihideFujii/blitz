

/*
A Graph describes a set of computations that are to be performed, 
as well as the dependencies between those computations.
*/

pub enum ConstructionContext {
  NotTracked,
  DirectSession,
  EagerRuntime
}

struct Node {}

impl Node {}

struct Edge {}

impl Edge {}

// Thread compatible but not thread safe. 
struct Graph {}

impl Graph {}