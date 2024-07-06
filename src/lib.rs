pub mod sort;


#[cfg(test)]
mod tests {

    #[test]
    fn test_sort(){
        let arr = vec![4,2,8,5,1];
        let res = crate::sort::bubble_sort(arr);
        assert_eq!(res,vec![1,2,4,5,8]);

        let opt_arr=vec![8,4,3,1,14,7];
        let opt_res = crate::sort::bubble_sort_optimal(opt_arr);
        assert_eq!(opt_res,vec![1,3,4,7,8,14]);
    }
}
