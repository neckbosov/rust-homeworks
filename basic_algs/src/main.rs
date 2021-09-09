fn max_element(arr: &[i32; 10]) -> i32 {
    let mut res = i32::MIN;
    for x in arr {
        if *x > res {
            res = *x;
        }
    }
    res
}

fn nth_prime(n: u32) -> u32 {
    let mut primes = Vec::new();
    for x in 2u32..u32::MAX {
        let mut is_prime = true;
        for d in &primes {
            if x % *d == 0 {
                is_prime = false;
                break;
            }
            if d * d > x {
                break;
            }
        }
        if is_prime {
            primes.push(x);
            if primes.len() >= n as usize {
                break;
            }
        }
    }
    *primes.last().unwrap()
}

fn find_num(arr: &[i32; 10], needle: &i32) -> Option<usize> {
    let mut lb = 0;
    let mut rb = arr.len();
    while lb < rb {
        let mid = (lb + rb) / 2;
        if arr[mid] == *needle {
            return Some(mid);
        } else if arr[mid] < *needle {
            lb = mid + 1;
        } else {
            rb = mid;
        }
    }
    None
}

fn find_and_print_res(arr: &[i32; 10], needle: &i32) {
    let find_res = find_num(&arr, &needle);
    if let Some(pos) = find_res {
        println!("Position of {} in array: {}", needle, pos);
    } else {
        println!("{} is not found in array", needle);
    }
}

fn main() {
    let mut arr = [3, 45, 2, 1, 4, -1, 2, -44, 9, 0];
    print!("Array: ");
    for elem in &arr {
        print!("{} ", *elem);
    }
    println!();
    println!("Max element in array: {}", max_element(&arr));
    arr.sort();
    print!("Sorted array: ");
    for elem in &arr {
        print!("{} ", *elem);
    }
    println!();
    find_and_print_res(&arr, &4);
    find_and_print_res(&arr, &5);
    let n = 325;
    println!("{}th prime number is {}", n, nth_prime(n));
}
