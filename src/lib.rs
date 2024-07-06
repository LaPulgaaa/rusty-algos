pub mod sort;


#[cfg(test)]
mod tests {

    #[test]
    fn test_sort(){
        let arr = vec![4,2,8,5,1];
        let res = crate::sort::bubble_sort(arr);
        assert_eq!(res,vec![1,2,4,5,8]);
    }
}
