pub struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

pub fn is_vector(t: Tuple) -> bool {
    return t.w == 0.0;
}

pub fn is_point(t: Tuple) -> bool {
    return t.w == 1.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_a_correct_tuple() {
        let t = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
        };
        assert_eq!(t.x, 4.3);
        assert_eq!(t.y, -4.2);
        assert_eq!(t.z, 3.1);
        assert_eq!(t.w, 1.0);
    }

    #[test]
    fn tuple_is_a_not_vector() {
        let t = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
        };
        assert_eq!(false, is_vector(t));
    }

    #[test]
    fn tuple_is_a_vector() {
        let t = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.0,
        };
        assert_eq!(true, is_vector(t));
    }

    #[test]
    fn tuple_is_not_a_point() {
        let t = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.0,
        };
        assert_eq!(false, is_point(t));
    }

    #[test]
    fn tuple_is_a_point() {
        let t = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
        };
        assert_eq!(true, is_point(t));
    }
}
