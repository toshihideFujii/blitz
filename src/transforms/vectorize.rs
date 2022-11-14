#![allow(dead_code)]

/*
This file defines prototypes for accessor functions
that expose passes in the Vectorize transformations
library.
*/

struct VectorizeConfig {}

pub fn create_loop_vectorize_pass() {}

pub fn create_slp_vectorizer_pass() {}

pub fn vectorize_basic_block() {}

pub fn create_load_store_vectorizer_pass() {}

pub fn create_vector_combine_pass() {}