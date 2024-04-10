/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T>(array: &mut [T])
where
    T: PartialOrd + Copy,
{
    //TODO
    let n = array.len();
    quick_sort(array, 0, n - 1);
}

fn quick_sort<T>(a: &mut [T], l: usize, r: usize)
where
    T: PartialOrd + Copy,
{
    if l <= r {
        let p = par(a, l, r);
        if p > 0 && l < p {
            quick_sort(a, l, p - 1);
        }
        if r > p {
            quick_sort(a, p + 1, r);
        }
    }
}

fn par<T>(a: &mut [T], l: usize, r: usize) -> usize
where
    T: PartialOrd + Copy,
{
    let p = a[r];
    let mut i = l;
    for j in l..r {
        if a[j] < p {
            a.swap(i, j);
            i += 1;
        }
    }
    a.swap(i, r);
    i
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
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}

