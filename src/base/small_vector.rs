// Minimal SmallVector implementation. Uses inline storage first, switches to
// malloc when it overflows.
//#[derive(Copy, Clone)]
struct SmallVector<T> {
    begin_: std::marker::PhantomData<T>,
    end_: std::marker::PhantomData<T>,
    //end_of_storage_: T,
}

impl<T> Copy for SmallVector<T> {}

impl<T> Clone for SmallVector<T> {
    fn clone(&self) -> Self {
        return *self;
    }
}

impl<T> SmallVector<T> {
    pub fn data() {}

    pub fn begin(&self) -> std::marker::PhantomData<T> {
        return self.begin_;
    }

    pub fn end(&self) -> std::marker::PhantomData<T> {
        return self.end_;
    }

    pub fn size(&self) {
        //return self.end_ - self.begin_;
    }

    pub fn empty(&self) -> bool {
        return self.begin_ == self.end_;
    }

    pub fn capacity() {}

    pub fn back() {}

    pub fn at(&self, _index: usize) {
        //self.begin_[index]
    }

    pub fn emplace_back() {}

    pub fn pop_back(&self, _count: usize) {
        //self.end_ -= count
    }

    pub fn resize_no_init() {}

    // Clear without freeing any storage.
    pub fn clear(&mut self) {
        //self.end_ = self.begin_
    }

    pub fn reset() {}

    fn grow() {}

    fn is_big() {}

    fn inline_storage_begin() {}
}
