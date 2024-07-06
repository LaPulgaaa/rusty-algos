pub fn bubble_sort(mut arr: Vec<i32>)->Vec<i32>{
    for i in 0..arr.len()-1{
        for j in 0..arr.len()-i-1{
            if arr[j]>arr[j+1]{
                let temp = arr[j];
                arr[j] = arr[j+1];
                arr[j+1] = temp;
            }
        }
    }
    arr
}

pub fn bubble_sort_optimal(mut arr: Vec<i32>)->Vec<i32>{
    let mut swapped = false;
    
    for i in 0..arr.len()-1{
        for j in 0..arr.len()-i-1{
            if arr[j]>arr[j+1]{
                swapped=true;
                let temp = arr[j];
                arr[j] = arr[j+1];
                arr[j+1] = temp;
            }
        }
        if swapped==false{
            return arr;
        }
    }

    arr

}