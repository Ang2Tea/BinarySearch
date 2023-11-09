fn binary_search(array: &[i32], target: i32) -> Option<i32> {
    
    let mut left = 0;
    let mut right = array.len() - 1;

    while left <= right {
        let mid = (right - left) / 2 + left;

        println!("Mid: {}", array[mid]);

        if target > array[mid] {
            left = mid + 1;
        }
        else if target < array[mid] {
            right = mid - 1;
        }
        else{
            return match mid.try_into() {
                Ok(result) => Some(result),
                Err(_) => None
            }
        }
    }

    return Some(-1);
}


fn main() {
    let array:[i32; 100] = (1..=100).collect::<Vec<_>>()
    .try_into()
    .expect("wrong size iterator");

    match binary_search(&array, 38) {
        Some(num) => println!("{}", num),
        None => println!("Error")
    }
}
