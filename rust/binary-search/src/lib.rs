/*
pub fn find(array: &[i32], key: i32) -> Option<usize> {

    let result = array.binary_search(&key);
    match result {
        Ok(index) => Some(index),
        Err(_) => None,
    }
}
*/

pub fn find<T, A>(array: A, key: T) -> Option<usize>
where
    T: std::cmp::Ord + Copy,
    A: AsRef<[T]>,
{
    let array = array.as_ref().to_vec();
    let result = array.binary_search(&key);
    match result {
        Ok(index) => Some(index),
        Err(_) => None,
    }
}
