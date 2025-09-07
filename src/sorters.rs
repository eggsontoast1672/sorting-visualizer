pub trait Sorter {
    fn step(&mut self, data: &mut [usize]) -> bool;
    fn pointers(&self) -> Vec<usize>;
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
            data.swap(self.current, self.current + 1);
        }

        self.current += 1;
        return false;
    }

    fn pointers(&self) -> Vec<usize> {
        vec![self.current]
    }
}

pub struct QuickSorter {
    stack: Vec<(usize, usize)>,
    current_low: usize,
    current_high: usize,
    left_pointer: usize,
    right_pointer: usize,
}

impl QuickSorter {
    pub fn new(data_length: usize) -> Self {
        Self {
            stack: Vec::new(),
            current_low: 0,
            current_high: data_length - 1,
            left_pointer: 0,
            right_pointer: 0,
        }
    }

    fn repartition(&mut self) -> bool {
        // Set up the new blocks to sort, if needed. We push the lower block last so that it
        // will be processed first.
        if self.left_pointer + 1 < self.current_high {
            self.stack.push((self.left_pointer + 1, self.current_high));
        }

        if self.current_low < self.left_pointer - 1 {
            self.stack.push((self.current_low, self.left_pointer - 1));
        }

        let Some((low, high)) = self.stack.pop() else {
            return true;
        };

        self.current_low = low;
        self.current_high = high;
        self.left_pointer = low;
        self.right_pointer = low;

        false
    }
}

impl Sorter for QuickSorter {
    fn step(&mut self, data: &mut [usize]) -> bool {
        // At this point, we have completed the partitioning step. Let's choose new low and high
        // and reset everything.
        if self.right_pointer >= self.current_high {
            data.swap(self.left_pointer, self.current_high);
            if self.repartition() {
                return true;
            }
        }

        if data[self.right_pointer] < data[self.current_high] {
            data.swap(self.left_pointer, self.right_pointer);
            self.left_pointer += 1;
        }

        self.right_pointer += 1;
        false
    }

    fn pointers(&self) -> Vec<usize> {
        vec![self.left_pointer, self.right_pointer]
    }
}
