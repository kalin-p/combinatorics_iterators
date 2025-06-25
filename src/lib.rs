mod permutation;
mod combination;

#[cfg(test)]
mod tests {
    use factorial::Factorial;
    use super::permutation::Permutations;

    #[test]
    fn test_permutations() {
        let v = vec![1, 2, 3, 4];

        let all_perms: Vec<Vec<i32>> = Permutations::from(v.clone()).collect();

        assert_eq!(all_perms.len(), v.len().factorial());

        for (i, el) in all_perms.clone().into_iter().enumerate() {
            for other in (i + 1)..all_perms.len() {
                assert_ne!(el, all_perms[other]);
            }
        }

    }

    use super::combination::Combinations;

    #[test]
    fn test_combinations() {
        let v = vec![1, 2, 3, 4, 5];
        for c in Combinations::new(v, 3).into_iter() {
            println!("{:?}", c);
        };
    }
}
