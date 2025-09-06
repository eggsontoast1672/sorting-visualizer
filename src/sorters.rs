pub trait Sorter {
    fn step(&mut self, data: &mut [usize]) -> bool;
    fn current(&self) -> usize;
}

pub struct BubbleSorter {
    current: usize,
    // Points to the index of the first element of the sorted block at the end of the slice.
    first_sorted: usize,
}

impl BubbleSorter {
    pub const fn new(data_length: usize) -> Self {
        Self {
            current: 0,
            first_sorted: data_length,
        }
    }
}

impl Sorter for BubbleSorter {
    fn step(&mut self, data: &mut [usize]) -> bool {
        // If the first index of the sorted block is the beginning of the slice, that means that
        // the entire slice is sorted and we are done.
        if self.first_sorted == 0 {
            return true;
        }

        // In this case, self.current + 1 would point to some element within the sorted block,
        // which we do not want to mess with. We can add one more to the sorted block.
        if self.current >= self.first_sorted - 1 {
            self.current = 0;
            self.first_sorted -= 1;
            // return false;
        }

        if data[self.current] > data[self.current + 1] {
            // SAFETY: the indices self.current and self.current + 1 cannot possibly be the same,
            // since no number is equal to itself plus 1.
            let [a, b] =
                unsafe { data.get_disjoint_unchecked_mut([self.current, self.current + 1]) };

            std::mem::swap(a, b);
        }

        self.current += 1;
        return false;
    }

    fn current(&self) -> usize {
        self.current
    }
}
