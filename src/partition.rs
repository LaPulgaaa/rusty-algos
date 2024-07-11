
pub fn count_inv(arr: &mut Vec<i32>, l: usize, r:usize)->i32{
    let mut ret:i32 =0;
    
    if l<r{
        let mid = (r-l)/2 + l;
        
        ret+=count_inv(arr,l,mid);
        ret+=count_inv(arr,mid+1,r);
        
        ret+=merge_and_count(arr,l,mid,r);
    }
    
    return ret;
}

fn merge_and_count(arr:&mut Vec<i32>, l: usize, mid: usize, r: usize) ->i32{
    
    let len1: usize = mid-l+1;
    let len2: usize = r-mid;
    
    let mut arr1 = vec![0; len1];
    arr1.copy_from_slice(&arr[l..mid+1]);
    
    let mut arr2 = vec![0; len2];
    arr2.copy_from_slice(&arr[mid+1..r+1]);
    
    println!("{:?} -- {:?}",arr1,arr2);
    
    let (mut i, mut j, mut k): (usize,usize,usize) = (0,0,l);
    
    let mut inversion:i32 = 0;
    
    while i<len1 && j<len2 {
        if arr1[i]<=arr2[j]{
            arr[k]=arr1[i];
            k+=1;
            i+=1;
        }
        else {
            arr[k]=arr2[j];
            j+=1;
            k+=1;
            
            inversion+= (len1-i) as i32;
        }
    }
    
    while i<len1{
        arr[k]=arr1[i];
        k+=1;
        i+=1;
    }
    
    while j<len2{
        arr[k]=arr2[j];
        k+=1;
        j+=1;
    }
    
    return inversion;
    
}

