/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn quick_sort<T: Ord + Copy>(a: &mut [T], mut l: usize, mut r: usize) {
     while l < r {
        let x = a[(l + r) >> 1];
        let mut i = l;
        let mut j = r;
        loop {
            while a[i] < x {
                i += 1;
            }
            while a[j] > x {
                if j == 0 {
                    break;
                }
                j -= 1;
            }
            if i >= j {
                break;
            }
            a.swap(i, j);
            i += 1;
            if j == 0 {
                break;
            }
            j -= 1;
        }
        if j.wrapping_sub(l) < r.wrapping_sub(j + 1) {
            quick_sort(a, l, j);
            l = j + 1;
        } else {
            quick_sort(a, j + 1, r);
            r = j;
        }
    }
}

pub fn sort<T: Ord + Copy>(array: &mut [T]) {
    if !array.is_empty() {
        let last = array.len() - 1;
        quick_sort(array, 0, last);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        println!("{:?}", vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}