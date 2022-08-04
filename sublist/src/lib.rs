#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub trait HasPart {
    fn has_part(&self, rhs: &Self) -> bool;
}

impl<T: PartialEq> HasPart for [T] {
    fn has_part(&self, second: &Self) -> bool {
        if second.is_empty() {
            return true;
        }
        self.windows(second.len()).any(|curr| curr == second)
    }
}


pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        Comparison::Equal
    } else if _first_list.has_part(_second_list) {
        Comparison::Superlist
    } else if _second_list.has_part(_first_list) {
        Comparison::Sublist
    } else {
        Comparison::Unequal
    }
}