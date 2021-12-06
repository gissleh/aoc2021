struct Permutation<T, const N: usize> {
    data: [T; N],
    stack: Vec<usize>,
    popped: bool,
}

impl<T, const N: usize> Iterator for Permutation<T, N>
    where T: Clone + Copy
{
    type Item = [T; N];

    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.len() == 0 {
            return None;
        }

        let from = self.stack.len() - 1;
        let to = N;
        let mut i = *self.stack.last().unwrap();

        if self.popped {
            self.data.swap(from, i);
            *self.stack.last_mut().unwrap() += 1;
            i += 1;
        }
        self.popped = false;

        if self.stack.len() == N {
            self.stack.pop();
            self.popped = true;

            Some(self.data)
        } else {
            if i == to {
                self.stack.pop();
                self.popped = true;

                self.next()
            } else {
                self.data.swap(from, i);
                self.stack.push(from + 1);

                self.next()
            }
        }
    }
}

pub fn permutations<T, const N: usize>(arr: [T; N]) -> impl Iterator<Item=[T; N]>
    where T: Clone + Copy {
    let mut stack = Vec::with_capacity(N);
    stack.push(0);

    Permutation {
        popped: false,
        data: arr,
        stack,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_permutations_count() {
        assert_eq!(permutations([1, 2, 3]).count(), 1 * 2 * 3);
        assert_eq!(permutations([1, 2, 3, 4]).count(), 1 * 2 * 3 * 4);
        assert_eq!(permutations([1, 2, 3, 4, 5]).count(), 1 * 2 * 3 * 4 * 5);
        assert_eq!(permutations([1, 2, 3, 4, 5, 6]).count(), 1 * 2 * 3 * 4 * 5 * 6);
        assert_eq!(permutations([1, 2, 3, 4, 5, 6, 7]).count(), 1 * 2 * 3 * 4 * 5 * 6 * 7);
        assert_eq!(permutations([1, 2, 3, 4, 5, 6, 7, 8]).count(), 1 * 2 * 3 * 4 * 5 * 6 * 7 * 8);
    }

    #[test]
    fn test_permutations_all() {
        let mut hs = HashSet::with_capacity(320);

        for v in permutations([1, 2, 3, 4, 5, 6, 7, 8, 9]) {
            if hs.contains(&v) {
                panic!("repeated permutation: {:?}", v);
            }
            hs.insert(v);
        }
    }
}