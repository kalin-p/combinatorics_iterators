mod permutation;

#[cfg(test)]
mod tests {
    use factorial::Factorial;
    use super::permutation::Permutations;

    #[test]
    fn test_permutations() {
        let v = vec![1, 2, 3, 4];
        // let perms = Permutation::from(v.clone());
        let all_perms: Vec<Vec<i32>> = Permutations::from(v.clone()).collect();

        assert_eq!(all_perms.len(), v.len().factorial());

        for (i, el) in all_perms.clone().into_iter().enumerate() {
            for other in (i + 1)..all_perms.len() {
                assert_ne!(el, all_perms[other]);
            }
        }

    }
}
