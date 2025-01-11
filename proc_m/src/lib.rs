#[allow(unused_imports)]
use comp_macro::comp;

#[cfg(test)]
mod tests {
    use comp_macro::comps;

    use super::*;

    #[test]
    fn comp_it_works() {
        let result = comp![x for x in [1,2,3]].collect::<Vec<_>>();
        assert_eq!(result, [1, 2, 3]);

        let result = comp![x*x for x in [1,2,3]].collect::<Vec<_>>();
        assert_eq!(result, [1, 4, 9]);

        let result = comp![(num * 2, String::from("hi") + t) for (num, t) in [(4, "hello"),(5, "world"),(6, "person")]].collect::<Vec<_>>();
        assert_eq!(
            result,
            [
                (8, String::from("hihello")),
                (10, String::from("hiworld")),
                (12, String::from("hiperson"))
            ]
        );

        let result = comp![num*8 for num in [2,3,4,5,7] if num % 2 == 0].collect::<Vec<_>>();
        assert_eq!(result, [16, 32]);
    }
    #[test]
    fn comps_it_works() {
        let vec_of_vecs = vec![vec![1, 2, 3], vec![4, 5, 6]];

        let result = comps![x for vec in vec_of_vecs for x in vec].collect::<Vec<_>>();
        assert_eq!(result, [1, 2, 3, 4, 5, 6]);
    }
}
