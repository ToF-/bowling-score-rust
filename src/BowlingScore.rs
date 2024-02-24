#[derive(PartialEq, Debug, Clone, Copy)]
struct Frame {
    throw1: i32,
    throw2: i32,
}

fn frames(throws: &Vec<i32>) -> Vec<Frame> {
    if throws.len() == 0 {
        vec!()
    } else {
        let mut result = vec![];
        result.insert(0, frame(throws[0], throws[1]));
        let mut next = frames(&throws[2..].to_vec());
        result.append(&mut next);
        result
    }
}

fn frame(throw1: i32, throw2: i32) -> Frame {
    Frame { throw1: throw1, throw2: throw2 }
}
#[cfg(test)]
    use speculoos::*;

    #[test]
    fn interpret_an_empty_list() {
        assert_that(&frames(&vec![])).is_equal_to(vec![]);
    }
    #[test]
    fn interpret_a_single_frame() {
        assert_that(&frames(&vec![3, 2])).is_equal_to(vec![frame(3,2)]);
        assert_that(&frames(&vec![7, 1])).is_equal_to(vec![frame(7,1)]);
    }
    #[test]
    fn interpret_several_frames() {
        assert_that(&frames(&vec![3, 2, 7, 1])).is_equal_to(vec![frame(3,2), frame(7,1)]);
    }
