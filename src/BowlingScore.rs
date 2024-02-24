#[derive(PartialEq, Debug, Clone, Copy)]
enum Frame {
    Normal { throw1: i32, throw2: i32, },
    Spare { throw: i32, },
}

fn frames(throws: &Vec<i32>) -> Vec<Frame> {
    if throws.len() == 0 {
        vec!()
    } else {
        let mut result = vec![];
        if throws[0] + throws[1] == 10 {
            result.insert(0, spare(throws[0]))
        } else {
            result.insert(0, normal(throws[0], throws[1]));
        }
        let mut next = frames(&throws[2..].to_vec());
        result.append(&mut next);
        result
    }
}

fn normal(throw1: i32, throw2: i32) -> Frame {
    Frame::Normal { throw1: throw1, throw2: throw2 }
}

fn spare(throw: i32) -> Frame {
    Frame::Spare { throw: throw }
}
#[cfg(test)]

    #[test]
    fn interpret_an_empty_list() {
        assert_eq!(&frames(&vec![]), &vec![]);
    }
    #[test]
    fn interpret_a_single_frame() {
        assert_eq!(&frames(&vec![3, 2]), &vec![normal(3,2)]);
        assert_eq!(&frames(&vec![7, 1]), &vec![normal(7,1)]);
    }
    #[test]
    fn interpret_several_frames() {
        assert_eq!(&frames(&vec![3, 2, 7, 1]), &vec![normal(3,2), normal(7,1)]);
    }
    #[test]
    fn interpret_a_spare() {
        assert_eq!(&frames(&vec![6,4]), &vec![spare(6)]);
    }
