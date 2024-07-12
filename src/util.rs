
pub fn swap(arr: &mut Vec<i32>, i: usize, j: usize){
    let temp = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;
}