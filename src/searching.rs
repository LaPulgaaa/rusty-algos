use crate::partition::lomutopart;

pub fn quick_select(arr: &mut [i32], k: usize)->i32{

    let (mut l, mut h):(usize,usize) = (0,arr.len()-1);

    while l<=h{
        let part = lomutopart(arr, l, h);

        if part == k-1 {
            return arr[part];
        }

        else if part < k-1 {
            l = part+1;
        }

        else {
            h = part-1;
        }
    }

    -1
}