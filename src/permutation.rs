use factorial::Factorial;

#[derive(Debug)]
pub struct Permutations<T>
where
    T: Clone
{
    v: Vec<T>,
    counter: usize,
    factorials: Vec<usize>
}

impl<T> From<Vec<T>> for Permutations<T>
where
    T: Clone
{
    fn from(value: Vec<T>) -> Self {
        let factorials = (1..=value.len())
            .map(|x| x.factorial())
            // for explanation of the reversal see the second paragraph below
            .rev()
            .collect();

        Permutations{
            v: value,
            counter: 0,
            factorials
        }
    }
}

// This is an iterative implementation of Heap's Algorithm for finding all
// permutations of a set (https://en.wikipedia.org/wiki/Heap's_algorithm). The
// original relies on recursion to swap elements in the set. To make it work
// iteratively I check if a counter is evenly divisible by N! where `0 <= N <
// set.len()`. If this condition is fulfilled the Nth members of the set is
// swapped with either the first item in the set or with one of the items that
// come before it. This depends on weather the subset of length N has an even or
// an odd number of elements.

// if counter % N! == counter % len! == 0 we are done iterating

// Since `counter % 1! == 0` is always true and `counter % 2! == 0` is true
// every other time we must check if N! divides the counter evenly in descending
// order of N. Hence `Permutation.factorials` is reversed. However this also
// means that `i` represents not the index, but `len - index`. Therefore when
// using `i` to index the set it should be flipped back with `len - i`.
impl<T> Iterator for Permutations<T>
where
    T: Clone
{
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        // We use a copy of the counter, because if `quot > 1` we need to only
        // pass on the value of `modulo` to the next iteration. Hence the
        // `tmp_counter = modulo` at the end of the loop.
        let mut tmp_counter = self.counter;
        for (i, fac) in self.factorials.iter().enumerate() {
            let modulo = tmp_counter % fac;
            let quot = tmp_counter / fac;
            let len = self.v.len();

            // only make a swap if the counter value is divided evenly by
            // `fac`. There will always be a swap in the end, because `fac =
            // self.factorials[n - 1] = 1`.
            if modulo == 0 && quot > 0 {
                if i == 0 {
                    // the only way to divide `len.factorial()` evenly is for
                    // `counter` to have reached it`
                    return None

                } else if (len - (i - 1)) % 2 == 0 {
                    // when manipulating a subset with an even nubmer of
                    // elements swap each element with the last in
                    // order. Explained here
                    // https://en.wikipedia.org/wiki/Heap's_algorithm#Details_of_the_algorithm
                    // The last item in a subset is given by `len - i`. In most
                    // cases when `modulo == 0` we have reached `fac == 1`,
                    // which means that `i == len - 1`. Then still we manipulate
                    // a subset of 2 elements.`len - (i - 1) = len - (len - 1
                    // -1) = 2` in case you were wondering where that expression
                    // comes from.

                    // we have made sure `quot - 1` will not be out of bounds by
                    // reducing the `tmp_counter` value at each step of the loop
                    self.v.swap(quot - 1,  len - i);
                    break;
                } else {
                    // With subsets of odd nubmer of elements always swap the first
                    // with the last
                    self.v.swap(0, len - i);
                    break;
                }
            }
            tmp_counter = modulo;
        }
        self.counter += 1;
        return Some(self.v.clone())
    }
}
