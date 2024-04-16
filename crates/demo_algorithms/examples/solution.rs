
struct Solution;

#[derive(Debug)]
struct Set<T> {
    data: Vec<T>,
}

impl<T: PartialEq> Set<T> {
    fn new() -> Self {
        Set { data: Vec::new() }
    }

    fn insert(&mut self, item: T) -> &mut Self {
        if !self.contains(&item) {
            self.data.push(item);
        }
        self
    }

    fn contains(&self, item: &T) -> bool {
        self.data.contains(item)
    }
}

#[derive(Debug)]
struct MinHeap<T> {
    data: Vec<T>,
}

impl<T: Ord> MinHeap<T> {
    fn new() -> Self {
        MinHeap { data: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.data.push(item);
        self.sift_up(self.data.len() - 1);
    }

    fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }

        let last_index = self.data.len() - 1;
        self.data.swap(0, last_index);
        let item = self.data.pop();

        self.sift_down(0);

        item
    }

    fn sift_up(&mut self, index: usize) {
        let mut child = index;
        while child > 0 {
            let parent = (child - 1) / 2;
            if self.data[child] < self.data[parent] {
                self.data.swap(child, parent);
                child = parent;
            } else {
                break;
            }
        }
    }

    fn sift_down(&mut self, index: usize) {
        let mut parent = index;
        loop {
            let left_child = 2 * parent + 1;
            let right_child = 2 * parent + 2;
            let mut smallest = parent;

            if left_child < self.data.len() && self.data[left_child] < self.data[smallest] {
                smallest = left_child;
            }

            if right_child < self.data.len() && self.data[right_child] < self.data[smallest] {
                smallest = right_child;
            }

            if smallest != parent {
                self.data.swap(parent, smallest);
                parent = smallest;
            } else {
                break;
            }
        }
    }

    fn peek(&self) -> Option<&T> {
        self.data.get(0)
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' => {
                    if stack.pop() != Some('(') {
                        return false;
                    }
                }
                ']' => {
                    if stack.pop() != Some('[') {
                        return false;
                    }
                }
                '}' => {
                    if stack.pop() != Some('{') {
                        return false;
                    }
                }
                _ => return false,
            }
        }

        stack.is_empty()
    }
}

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut i = matrix.len() as i32 - 1;
        let mut j = 0;

        while i >= 0 && j < matrix[i as usize].len() {
            if matrix[i as usize][j] > target {
                i -= 1;
            } else if matrix[i as usize][j] < target {
                j += 1;
            } else {
                return true;
            }
        }
        false
    }
}

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i64 {
        let mut i = 1;
        let mut seen = Set::new();
        let mut ugly = 1;
        let mut min_heap = MinHeap::new();
        min_heap.push(1);
        seen.insert(1);

        while i <= n as usize {
            ugly = min_heap.pop().unwrap();
            i += 1;

            for factor in [2, 3, 5].iter() {
                let prime = (*factor as i64) * (ugly as i64);

                if !seen.contains(&prime) {
                    seen.insert(prime);
                    min_heap.push(prime);
                }
            }
        }

        ugly as i64
    }
}



fn main() {
    Solution::nth_ugly_number(1407);
}
