pub mod sort;
pub mod partition;

#[cfg(test)]
mod tests {

    #[test]
    fn test_sort(){
        let arr = vec![4,2,8,5,1];
        let res = crate::sort::bubble_sort(arr.clone());
        assert_eq!(res,vec![1,2,4,5,8]);

        let opt_arr=vec![8,4,3,1,14,7];
        let opt_res = crate::sort::bubble_sort_optimal(opt_arr);
        assert_eq!(opt_res,vec![1,3,4,7,8,14]);

        let sel_arr = crate::sort::select_sort(arr.clone());
        assert_eq!(sel_arr, vec![1,2,4,5,8]);

        let sel_opt_arr = crate::sort::select_sort_opt(arr.clone());
        assert_eq!(sel_opt_arr, vec![1,2,4,5,8]);

        let ins_arr= crate::sort::insertion_sort(arr.clone());
        assert_eq!(ins_arr, vec![1,2,4,5,8]);

        // let ret = crate::sort::merge_sort(vec![2,5,9,13,20,3,4,7,15,16],0,9);
        // assert_eq!(ret,vec![2,3,4,5,7,9,13,15,16,20]);
    }

    #[test]
    fn test_partition(){
        let mut arr = vec![2,4,1,3,5];
        let len = arr.len();
        let num_of_inv = crate::partition::count_inv(&mut arr, 0, len-1);
        assert_eq!(num_of_inv,3);
        assert_eq!(arr, vec![1,2,3,4,5]);
    }
}
