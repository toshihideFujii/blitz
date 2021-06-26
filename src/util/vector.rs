/*
use generic_array::{ArrayLength, GenericArray};

struct Vector<T, N: ArrayLength<usize>> {
    start_: GenericArray<T, N>,
    length_: usize
}
*/

struct Vector<T> {
    start_: T,
    length_: usize,
}

impl<T> Vector<T> {
    pub fn new(data: T, length: usize) -> Vector<T> {
        Vector {
            start_: data,
            length_: length,
        }
    }

    // Returns the length of the vector.
    // Only use this if you really need an integer return value.
    // Use {size()} otherwise.
    pub fn length(self) -> usize {
        return self.length_;
    }

    // Reutrns the length of the vector as a size_t.
    pub fn size(self) -> usize {
        return self.length_;
    }

    // Returns whether or not the vector is empty.
    pub fn empty(self) -> bool {
        return self.length_ == 0;
    }
    /*
    pub fn at(self, index: usize) -> T {
        return self.start_[index]
    }
    */
    pub fn first() {}

    pub fn last() {}

    // Returns a pointer to the start of the data in the vector.
    pub fn begin() {}

    // For consistency with other containers. do also provide a {data} accessor.
    pub fn data(self) -> T {
        return self.start_;
    }

    // Returns a pointer past the end of the data in the vector.
    pub fn end() {}

    // Returns a clone of this vector with a new backing store.
    pub fn clone() {}

    pub fn truncate() {}

    // Releases the array underlying this vector.
    // Once disposed the vector is empty.
    pub fn dispose() {}
}
