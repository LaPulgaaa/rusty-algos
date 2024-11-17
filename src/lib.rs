pub mod misc;
pub mod partition;
pub mod searching;
pub mod sort;
pub mod strings;

#[cfg(test)]
mod tests {

    #[test]
    fn test_sort() {
        let arr = vec![4, 2, 8, 5, 1];
        let res = crate::sort::bubble_sort(arr.clone());
        assert_eq!(res, vec![1, 2, 4, 5, 8]);

        let opt_arr = vec![8, 4, 3, 1, 14, 7];
        let opt_res = crate::sort::bubble_sort_optimal(opt_arr);
        assert_eq!(opt_res, vec![1, 3, 4, 7, 8, 14]);

        let sel_arr = crate::sort::select_sort(arr.clone());
        assert_eq!(sel_arr, vec![1, 2, 4, 5, 8]);

        let sel_opt_arr = crate::sort::select_sort_opt(arr.clone());
        assert_eq!(sel_opt_arr, vec![1, 2, 4, 5, 8]);

        let ins_arr = crate::sort::insertion_sort(arr.clone());
        assert_eq!(ins_arr, vec![1, 2, 4, 5, 8]);

        let ret = crate::sort::merge_sort(vec![2, 5, 9, 13, 20, 3, 4, 7, 15, 16], 0, 9);
        assert_eq!(
            ret,
            vec![2, 3, 4, 5, 7, 9, 13, 15, 16, 20],
            "merge sort testing"
        );

        let mut lomuto_arr = vec![6, 5, 3, 1, 8, 7, 6, 4];
        let len = lomuto_arr.len();
        crate::sort::quick_sort_lomuto(&mut lomuto_arr, 0, len - 1);

        assert_eq!(lomuto_arr, vec![1, 3, 4, 5, 6, 6, 7, 8]);

        let mut hoare_arr = vec![6, 5, 3, 1, 8, 7, 6, 4];
        crate::sort::quick_sort_hoare(&mut hoare_arr, 0, 7);
        assert_eq!(hoare_arr, vec![1, 3, 4, 5, 6, 6, 7, 8]);

        let mut dutch_arr = vec![2, 0, 2, 1, 1, 0];
        crate::sort::dutch_national_flag_algo(&mut dutch_arr);
        assert_eq!(dutch_arr, [0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn test_partition() {
        let mut arr = vec![2, 4, 1, 3, 5];
        let len = arr.len();
        let num_of_inv = crate::partition::count_inv(&mut arr, 0, len - 1);
        assert_eq!(num_of_inv, 3);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);

        let mut arr2 = vec![3, 8, 6, 12, 10, 7];
        let pivot = crate::partition::lomutopart(&mut arr2, 0, 5);
        assert_eq!(pivot, 2);
        assert_eq!(arr2, vec![3, 6, 7, 12, 10, 8]);
    }

    #[test]
    fn test_searching() {
        let mut arr = vec![3, 2, 3, 1, 2, 4, 5, 5, 6];
        let k: usize = 9;
        let kthsmallest = 6;
        assert_eq!(crate::searching::quick_select(&mut arr, k), kthsmallest);

        // boyer-moore's algorithm

        assert_eq!(
            crate::searching::get_majority(&[4, 7, 0, 5, 3, 4, 6, 4]),
            -1
        );
        assert_eq!(crate::searching::get_majority(&[4, 7, 4, 4, 3, 4, 6, 4]), 4);
    }

    #[test]
    fn test_misc() {
        // min difference in an array
        assert_eq!(crate::misc::min_diff(&mut [1, 8, 12, 5, 18]), 3);
        assert_eq!(crate::misc::min_diff(&mut [4, 9, 1, 32, 13]), 3);
        assert_ne!(crate::misc::min_diff(&mut [1, 2, 3, 1, 5]), 1);
    }

    #[test]
    fn test_problems() {
        assert_eq!(
            crate::sort::choco_dist(&mut [3, 4, 1, 9, 56, 7, 9, 12], 5),
            6
        );
        assert_eq!(crate::sort::choco_dist(&mut [7, 3, 2, 4, 9, 12, 56], 3), 2);
        assert_eq!(crate::sort::choco_dist(&mut [5, 3, 2, 8, 4], 7), -1);

        assert_eq!(
            crate::misc::frequency_sort(String::from("cccaaab")),
            String::from("aaacccb")
        );

        assert_eq!(
            crate::misc::remove_outer_parentheses(String::from("(()())(())(()(()))")),
            String::from("()()()()(())")
        );

        assert_eq!(
            crate::misc::remove_outer_parentheses_one_liner(String::from("(()())(())(()(()))")),
            String::from("()()()()(())")
        );

        assert_eq!(
            crate::misc::longest_common_prefix(vec![
                String::from("flower"),
                String::from("flow"),
                String::from("flight")
            ]),
            String::from("fl")
        );

        assert_eq!(
            crate::misc::longest_common_prefix_one_liner(vec![
                String::from("flower"),
                String::from("flow"),
                String::from("flight")
            ]),
            String::from("fl")
        );

        assert_eq!(
            crate::misc::max_depth(String::from("(1+(2*3)+((8)/4))+1")),
            3
        );

        assert_eq!(crate::misc::my_atoi(String::from("1337c0d3")), 1337);

        assert_eq!(
            crate::misc::longest_palindrome(String::from("xynitin")),
            String::from("nitin")
        );

        assert_eq!(
            crate::misc::longest_palindrome(String::from("varun")),
            String::from("v")
        );

        assert_eq!(crate::misc::subarray_sum(vec![-1, 1, 0], 0), 3);

        assert_eq!(crate::misc::max_product(vec![-2, 3, -4]), 24);

        assert_eq!(crate::misc::sum_subarray_mins(vec![3, 1, 2, 4]), 17);

        assert_eq!(crate::misc::asteroid_collision(vec![10, 2, -5]), vec![10]);

        assert_eq!(
            crate::misc::asteroid_collision(vec![-2, -1, 1, 2]),
            vec![-2, -1, 1, 2]
        );

        assert_eq!(
            crate::misc::remove_kdigits(String::from("1432219"), 3),
            String::from("1219")
        );

        assert_eq!(
            crate::misc::remove_kdigits(String::from("10200"), 1),
            String::from("200")
        );

        assert_eq!(
            crate::misc::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]),
            6
        );

        assert_eq!(
            crate::misc::three_sum(vec![-4, -2, 1, -5, -4, -4, 4, -2, 0, 4, 0, -2, 3, 1, -5, 0]),
            6
        );

        assert_eq!(
            crate::misc::find_max_length(vec![0, 1, 0, 0, 1, 0, 1, 1]),
            8
        );

        assert_eq!(
            crate::misc::length_of_longest_substring(String::from("dvdf")),
            3
        );

        assert_eq!(
            crate::misc::binary_subarr_with_sum(vec![0, 0, 0, 0, 0], 0),
            15
        );

        assert_eq!(
            crate::misc::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        )
    }

    #[test]
    fn test_strings() {
        assert!(crate::strings::check_palindrome(String::from(
            "A man, a plan, a canal: Panama"
        )));
        assert!(!crate::strings::check_palindrome(String::from(
            "race a car"
        )));

        assert_eq!(
            crate::strings::reverse(String::from("the sky is blue")),
            String::from("blue is sky the")
        );

        assert!(!crate::strings::is_subsequence(
            String::from("axc"),
            String::from("ahbgdc")
        ));
        assert!(crate::strings::is_subsequence(
            String::from("AB"),
            String::from("ABCD")
        ));

        assert!(crate::strings::anagram(
            String::from("listen"),
            String::from("silent")
        ));
        assert!(!crate::strings::anagram(
            String::from("cat"),
            String::from("rat")
        ));
        assert_eq!(
            crate::strings::left_most_repeating_index(String::from("abccbaacz")),
            2
        );
        assert_eq!(
            crate::strings::left_most_repeating_index(String::from("abcdd")),
            3
        );
        assert_eq!(
            crate::strings::left_most_repeating_char(String::from("abccbaacz")),
            'c'
        );
        assert_eq!(
            crate::strings::left_most_repeating_char(String::from("abcdd")),
            'd'
        );
        assert_eq!(
            crate::strings::left_most_non_repeating(String::from("aabb")),
            -1
        );
        assert_eq!(
            crate::strings::left_most_non_repeating(String::from("leetcode")),
            0
        );
    }

    #[test]
    fn text_search() {
        assert_eq!(
            crate::strings::naive_search(String::from("ABABABCD"), String::from("ABAB")),
            [0, 2]
        );
        assert_eq!(
            crate::strings::naive_search(String::from("AAAAA"), String::from("AAA")),
            [0, 1, 2]
        );

        // -- RabinKarp --
        assert_eq!(
            crate::strings::rabin_karp(String::from("❤"), String::from("Rust is a systems programming language ❤ focused on safety, speed, and concurrency ❤. Rust has been gaining popularity due to its ability to prevent memory safety issues and provide zero-cost abstractions. Developers are choosing Rust for building reliable and efficient software, and Rust's ownership model ensures that data races are minimized ❤. Rust's ecosystem is growing, with a strong community contributing to its libraries and tools ❤. If you're looking to learn a new programming language, Rust might be an excellent choice because Rust offers a unique combination of power and safety.")),
            4
        );
    }
}
