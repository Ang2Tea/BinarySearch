use std::num::TryFromIntError;

enum ErrorKind {
    TryFromIntError(TryFromIntError),
    NotFound
}

fn binary_search(array: &[i32], target: i32) -> Result<i32, ErrorKind> {
    
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
                Ok(result) => Ok(result),
                Err(err) => Err(ErrorKind::TryFromIntError(err))
            }
        }
    }

    return Err(ErrorKind::NotFound);
}


fn main() {
    let array:[i32; 100] = (1..=100).collect::<Vec<_>>()
    .try_into()
    .expect("wrong size iterator");

    match binary_search(&array, 101) {
        Ok(num) => println!("{}", num),
        Err(err) => match err {
            ErrorKind::NotFound => println!("Число не найдено"),
            ErrorKind::TryFromIntError(_) => println!("Не удалось за мапить число")
        }
    }
}
