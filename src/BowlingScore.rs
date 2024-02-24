#[derive(PartialEq, Debug, Clone, Copy)]
struct Frame {
}
fn interpret(_throws: &Vec<i32>) -> Vec<Frame> {
    vec!()
}
#[cfg(test)]
    use speculoos::*;

    #[test]
    fn interpret_an_empty_list() {
        assert_that(&interpret(&vec![])).is_equal_to(vec![]);
    }
