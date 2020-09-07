pub mod dune {
    #[derive(Default)]
    pub struct MinHeap<T> {
        ary: Vec<T>,
    }

    impl<T: Ord> MinHeap<T> {
        pub fn with_capacity(capacity: usize) -> MinHeap<T> {
            MinHeap {
                ary: Vec::with_capacity(capacity),
            }
        }

        pub fn from_vec(ary: Vec<T>) -> MinHeap<T> {
            let mut unbuilt = MinHeap { ary };
            unbuilt.build();
            unbuilt
        }

        pub fn pop(&mut self) -> Option<T> {
            if self.is_empty() {
                None
            } else {
                let elem = self.ary.swap_remove(0);
                self.bubble_down(0);
                Some(elem)
            }
        }

        pub fn append(&mut self, other: &mut MinHeap<T>) {
            self.ary.append(&mut other.ary)
        }

        pub fn push(&mut self, item: T) {
            self.ary.push(item);
            let index = self.bubble_up();
            self.bubble_down(index);
        }

        pub fn peek(&self) -> Option<&T> {
            self.ary.first()
        }

        pub fn capacity(&self) -> usize {
            self.ary.capacity()
        }

        pub fn reserve_exact(&mut self, additional: usize) {
            self.ary.reserve_exact(additional)
        }

        pub fn reserve(&mut self, additional: usize) {
            self.ary.reserve(additional)
        }

        pub fn shrink_to_fit(&mut self) {
            self.ary.shrink_to_fit()
        }

        pub fn into_vec(self) -> Vec<T> {
            self.ary
        }

        pub fn len(&self) -> usize {
            self.ary.len()
        }

        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        pub fn clear(&mut self) {
            self.ary.clear()
        }

        fn bubble_up(&mut self) -> usize {
            let mut index = self.ary.len() - 1;
            while index > 0 {
                let parent = (index - 1) / 2;
                unsafe {
                    if self.ary.get_unchecked(index) >= self.ary.get_unchecked(parent) {
                        break;
                    }
                }
                self.ary.swap(index, parent);
                index = parent;
            }
            index
        }

        fn bubble_down(&mut self, start: usize) {
            let mut index = start;

            loop {
                let left = 2 * index + 1;
                let right = left + 1;

                let mut smallest = match self.ary.get(left) {
                    Some(child) => (child, left),
                    None => break,
                };

                if let Some(child) = self.ary.get(right) {
                    smallest = std::cmp::min(smallest, (child, right))
                }

                if self.ary[index] <= *smallest.0 {
                    break;
                }

                let (_, child_index) = smallest;

                self.ary.swap(index, child_index);
                index = child_index;
            }
        }

        fn build(&mut self) {
            if self.len() <= 1 {
                return;
            }

            let rightmost_branch = (self.len().next_power_of_two() >> 1) - 2;
            for index in (0..rightmost_branch).rev() {
                self.bubble_down(index)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dune::MinHeap;

    #[test]
    fn with_capacity_initializes_with_capacity() {
        for cap in 0..4096 {
            let heap = MinHeap::<()>::with_capacity(cap);
            assert!(heap.len() == 0);
            assert!(heap.is_empty());
            assert!(heap.capacity() >= cap);
        }
    }
}
