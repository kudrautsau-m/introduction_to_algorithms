pub enum SortOrder {
    Nonincreasing,
    Nondecreasing,
}

pub fn insertion_sort<T: PartialOrd>(items: &mut [T], sort_order: SortOrder) {
    for i in 1..items.len() {
        let mut j = i;

        let comparator = match sort_order {
            SortOrder::Nondecreasing => |a: &T, b: &T| a < b,
            SortOrder::Nonincreasing => |a: &T, b: &T| a > b,
        };

        while j > 0 && comparator(&items[j], &items[j - 1]) {
            items.swap(j - 1, j);
            j -= 1;
        }
    }
}

pub fn linear_search<T: PartialEq>(items: &[T], item: T) -> Option<usize> {
    for i in 0..items.len() {
        if items[i] == item {
            return Some(i);
        }
    }

    None
}

pub fn add_two_binary_integers(a: &[bool], b: &[bool]) -> Vec<bool> {
    assert_eq!(a.len(), b.len(), "Slices have different length.");

    let mut result = Vec::with_capacity(a.len() + 1);

    let mut carry = false;

    for i in (0..a.len()).rev() {
        result.push(
            (a[i] && !b[i] && !carry)
                || (b[i] && !a[i] && !carry)
                || (carry && !a[i] && !b[i])
                || (a[i] && b[i] && carry),
        );
        carry = (a[i] && (b[i] || carry)) || (b[i] && carry);
    }

    result.push(carry);

    result.reverse();

    result
}

pub fn selection_sort<T: PartialOrd>(items: &mut [T], sort_order: SortOrder) {
    let comparator = match sort_order {
        SortOrder::Nondecreasing => |a: &T, b: &T| a < b,
        SortOrder::Nonincreasing => |a: &T, b: &T| a > b,
    };

    for i in 1..items.len() {
        let mut swap_index = i - 1;

        for j in i..items.len() {
            if comparator(&items[j], &items[swap_index]) {
                swap_index = j;
            }
        }
        items.swap(swap_index, i - 1);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn insertion_sort_nondecreasing() {
        let mut array = [5, 2, 4, 6, 1, 3];
        insertion_sort(&mut array, SortOrder::Nondecreasing);
        assert_eq!(array, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn insertion_sort_nonincreasing() {
        let mut array = [5, 2, 4, 6, 1, 3];
        insertion_sort(&mut array, SortOrder::Nonincreasing);
        assert_eq!(array, [6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn insertion_sort_negative() {
        let mut array = [5, 0, -8, -6, -1, 1];
        insertion_sort(&mut array, SortOrder::Nondecreasing);
        assert_eq!(array, [-8, -6, -1, 0, 1, 5]);
    }

    #[test]
    fn insertion_sort_no_elements() {
        let mut array: [i32; 0] = [];
        insertion_sort(&mut array, SortOrder::Nondecreasing);
        assert_eq!(array, []);
    }

    #[test]
    fn insertion_sort_one_element() {
        let mut array = [1];
        insertion_sort(&mut array, SortOrder::Nondecreasing);
        assert_eq!(array, [1]);
    }

    #[test]
    fn linear_search_basic() {
        let array = [5, 2, 4, 6, 1, 3];

        assert_eq!(linear_search(&array, 4), Some(2));
    }

    #[test]
    fn linear_search_no_element() {
        let array = [5, 2, 4, 6, 1, 3];

        assert_eq!(linear_search(&array, 9), None);
    }

    #[test]
    fn add_two_binary_integers_basic() {
        let a = [false, false, false, false];
        let b = [false, false, false, false];

        assert_eq!(
            add_two_binary_integers(&a, &b),
            &[false, false, false, false, false]
        );

        let a = [false, false, false, true];
        let b = [false, false, false, false];

        assert_eq!(
            add_two_binary_integers(&a, &b),
            &[false, false, false, false, true]
        );

        let a = [false, false, false, true];
        let b = [false, false, false, true];

        assert_eq!(
            add_two_binary_integers(&a, &b),
            &[false, false, false, true, false]
        );

        let a = [false, false, true, true];
        let b = [false, true, true, true];

        assert_eq!(
            add_two_binary_integers(&a, &b),
            &[false, true, false, true, false]
        );

        let a = [false, false, false, true];
        let b = [true, true, true, true];

        assert_eq!(
            add_two_binary_integers(&a, &b),
            &[true, false, false, false, false]
        );
    }

    #[test]
    fn selection_sort_nondecreasing() {
        let mut array = [5, 2, 4, 6, 1, 3];
        selection_sort(&mut array, SortOrder::Nondecreasing);
        assert_eq!(array, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn selection_sort_nonincreasing() {
        let mut array = [5, 2, 4, 6, 1, 3];
        selection_sort(&mut array, SortOrder::Nonincreasing);
        assert_eq!(array, [6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn selection_sort_no_elements() {
        let mut array: [i32; 0] = [];
        selection_sort(&mut array, SortOrder::Nondecreasing);
        assert_eq!(array, []);
    }

    #[test]
    fn selection_sort_one_element() {
        let mut array = [1];
        selection_sort(&mut array, SortOrder::Nondecreasing);
        assert_eq!(array, [1]);
    }
}
