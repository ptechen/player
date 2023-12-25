pub mod dropdowns;
pub mod event;
pub mod input;
pub mod navs;
pub mod not_found;
pub mod select;
pub mod timestamp_interval;
pub mod textarea;
pub mod menu;
pub mod a;
pub mod ul;
pub mod alert;
pub mod li;
pub mod card;
pub mod lis;
pub mod list_group;
pub mod button;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
