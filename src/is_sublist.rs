#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Comparison {
    Equal,     // список `a` равен списку `b`
    Sublist,   // список `a` является подсписком `b`
    Superlist, // список `b` является подсписком `a`
    Other,     // списки не равны и не являются подсписками друг друга
}

// Knuth-Moris-Pratt algorithm for prefix function calculation and substring search
fn is_sublist<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    let mut prefix_function = Vec::with_capacity(a.len() + 1);
    let mut last_pf_val: usize = 0;
    if !a.is_empty() {
        prefix_function.push(0);
    }
    for i in 1..a.len() {
        while last_pf_val > 0 && a[i] != a[last_pf_val] {
            last_pf_val = prefix_function[last_pf_val - 1];
        }
        if a[last_pf_val] == a[i] {
            last_pf_val += 1;
        }
        prefix_function.push(last_pf_val);
    }
    prefix_function.push(0);
    last_pf_val = 0;
    for elem in b {
        while last_pf_val > 0 && *elem != a[last_pf_val] {
            last_pf_val = prefix_function[last_pf_val - 1];
        }
        if last_pf_val < a.len() && a[last_pf_val] == *elem {
            last_pf_val += 1;
        }
        if last_pf_val == a.len() {
            return true;
        }
    }
    false
}

pub fn compare<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    if a == b {
        Comparison::Equal
    } else if is_sublist(a, b) {
        Comparison::Sublist
    } else if is_sublist(b, a) {
        Comparison::Superlist
    } else {
        Comparison::Other
    }
}

#[cfg(test)]
mod tests {
    use crate::is_sublist::{compare, Comparison};

    #[test]
    fn test_equal_lists() {
        let a = [1, 2, 3, 4, 1];
        assert_eq!(compare(&a, &a), Comparison::Equal);

        let a: [i32; 0] = [];
        assert_eq!(compare(&a, &a), Comparison::Equal);
    }

    #[test]
    fn test_equal_sets_but_not_lists() {
        let a = [1, 2, 3];
        let b = [3, 2, 1];

        assert_eq!(compare(&a, &b), Comparison::Other);
    }

    #[test]
    fn test_non_equal() {
        let a = [1, 2, 3];
        let b = [3, 2, 1, 4, 5];

        assert_eq!(compare(&a, &b), Comparison::Other);
    }

    #[test]
    fn test_sublist_and_superlist() {
        let a = [1, 2, 3];
        let b = [1, 2, 3, 4, 5];
        assert_eq!(compare(&a, &b), Comparison::Sublist);
        assert_eq!(compare(&b, &a), Comparison::Superlist);

        let a = [3, 4, 5];
        let b = [1, 2, 3, 4, 5];
        assert_eq!(compare(&a, &b), Comparison::Sublist);
        assert_eq!(compare(&b, &a), Comparison::Superlist);

        let a = [3, 4];
        let b = [1, 2, 3, 4, 5];
        assert_eq!(compare(&a, &b), Comparison::Sublist);
        assert_eq!(compare(&b, &a), Comparison::Superlist);

        let a = [];
        let b = [1, 2, 3, 4, 5];
        assert_eq!(compare(&a, &b), Comparison::Sublist);
        assert_eq!(compare(&b, &a), Comparison::Superlist);
    }
}
