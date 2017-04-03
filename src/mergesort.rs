use std::clone::Clone;
use std::fmt::Debug;
use std::cmp::PartialOrd;

fn merge_sort<T:PartialOrd+Clone+Debug>(arr: &mut Vec<T>, start: usize, end: usize) -> &mut Vec<T> {
    if start < end {
        let mid: usize = (start + end)/2;
        merge_sort(arr, start, mid);
        merge_sort(arr, mid+1, end);
        merge(arr, start, mid, end);
    }
    arr
}

fn merge<T:PartialOrd+Clone+Debug>(arr1: &mut Vec<T>, start: usize, mid: usize, end: usize) {
    let mut i: usize = start;
    let mut j: usize = mid + 1;
    let mut arr2: Vec<T> = Vec::new();

    while i <= mid && j <= end {
        if arr1[i] <= arr1[j] {
	    arr2.push(arr1[i].clone());
	    i = i+1;
        }
        else {
	    arr2.push(arr1[j].clone());
	    j = j+1;
        }
    }
    while i <= mid {
	arr2.push(arr1[i].clone());
	i = i + 1;
    }
    while j <= end {
        arr2.push(arr1[j].clone());
	j = j + 1;
    }

    for k in start..(end+1) {
	arr1[k] = arr2[k-start].clone();	
    }
}

fn main() {
   let mut a: Vec<i32> = vec![5, 3, 20, 4, 1];
   let size: usize = a.len() - 1;
   let a_ref: &mut Vec<i32> = &mut a;
   println!("{:?}", a_ref);
   let b: &mut Vec<i32> = merge_sort(a_ref, 0, size);
   println!("{:?}", b);
}
