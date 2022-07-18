mod bubble_sort;
mod bogo_sort;
mod bucket_sort;
mod insertion_sort;

use std::cmp;
pub use self::bubble_sort::bubble_sort;
pub use self::insertion_sort::insertion_sort;
pub fn is_sorted<T>(arr: &[T]) -> bool
    where
        T: cmp::PartialOrd,
{
    if arr.is_empty() {
        return true;
    }

    let mut prev = &arr[0];

    for item in arr.iter().skip(1) {
        if prev > item {
            return false;
        }

        prev = item;
    }

    true
}
