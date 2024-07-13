use crate::partition::{lomutopart, hoare};

// O(N^2) O(1) stable
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

// O(N^2) O(1) stable
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

// O(N^2) O(len) unstable
pub fn select_sort(mut arr: Vec<i32>)->Vec<i32>{
    
    let len :usize= arr.len();

    if len==1  || len==0 {
        return arr;
    }

    let mut temp = Vec::new();

    let mut i=0;

    while i < len{
        let mut min_index = 0;
        for j in 1..arr.len(){
            if arr[min_index]>arr[j] {
                min_index = j;
            }
        }

        temp.push(arr[min_index]);
        arr[min_index]=i32::MAX;
        i += 1;
    }

    return temp;
}

// O(N^2) O(1) unstable
pub fn select_sort_opt(mut arr: Vec<i32>)->Vec<i32>{

    if arr.len()==0 || arr.len()==1 {
        return arr;
    }

    for i in 0..arr.len(){ 
        let mut rest_min=i;

        for j in i+1..arr.len(){
            if arr[j]<arr[rest_min]{
                rest_min = j;
            }
        }

        let temp = arr[rest_min];
        arr[rest_min] = arr[i];
        arr[i] = temp;
    }

    return arr;
}

// O(N^2) worst case O(N) best case -- O(1)
pub fn insertion_sort(mut arr: Vec<i32>)->Vec<i32>{

    if arr.len()==0 || arr.len()==1 {
        return arr;
    }
    
    for i in 1..arr.len(){
        let key = arr[i];
        let mut j = i;
        
        while arr[j-1]>key{
            arr[j] = arr[j-1];
            j-=1;
            
            if j==0{
                break;
            }
        }
        
        arr[j] = key;
    }
    
    return arr;
}

// O(NlogN) average/best O(N^2) worst case
pub fn quick_sort_lomuto(arr: &mut Vec<i32>, l: usize, h:usize){
    if l<h {
        let part = lomutopart(arr, l, h);
        quick_sort_lomuto(arr, l, part-1);
        quick_sort_lomuto(arr, part+1, h);
    }
}

pub fn quick_sort_hoare(arr: &mut Vec<i32>, l: usize, h:usize){
    if l<h {
        let part = hoare(arr, l, h);
        quick_sort_hoare(arr, l, part);
        quick_sort_hoare(arr, part+1, h);
    }
}