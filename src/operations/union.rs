use crate::{BooleanResult, Clipper, ClipperError, FillRule, Paths, PointScaler};

/// This function joins a set of closed subject paths, with and without clip
/// paths.
///
/// # Examples
///
/// ```rust
/// use clipper2::*;
///
/// let path_a = vec![(0.2, 0.2), (6.0, 0.2), (6.0, 6.0), (0.2, 6.0)];
/// let path_b = vec![(5.0, 5.0), (8.0, 5.0), (8.0, 8.0), (5.0, 8.0)];
///
/// let result = union::<Centi>(path_a, path_b, FillRule::default())
///     .expect("Failed to run boolean operation");
/// let output: Vec<Vec<(f64, f64)>> = result.closed.into();
/// let open_output: Vec<Vec<(f64, f64)>> = result.open.into();
///
/// dbg!(output, open_output);
/// ```
/// ![Image displaying the result of the union example](https://raw.githubusercontent.com/tirithen/clipper2/main/doc-assets/union.png)
///
/// For more details see the original [union](https://www.angusj.com/clipper2/Docs/Units/Clipper/Functions/Union.htm) docs.
pub fn union<P: PointScaler>(
    subject: impl Into<Paths<P>>,
    clip: impl Into<Paths<P>>,
    fill_rule: FillRule,
) -> Result<BooleanResult<P>, ClipperError> {
    Clipper::new()
        .add_subject(subject)
        .add_clip(clip)
        .union(fill_rule)
}

#[cfg(test)]
mod test {
    use crate::Centi;

    use super::*;

    #[test]
    fn test_union() {
        let path1 = vec![(0.2, 0.2), (6.0, 0.2), (6.0, 6.0), (0.2, 6.0)];
        let path2 = vec![(5.0, 5.0), (8.0, 5.0), (8.0, 8.0), (5.0, 8.0)];
        let expected_output = vec![vec![
            (6.0, 5.0),
            (8.0, 5.0),
            (8.0, 8.0),
            (5.0, 8.0),
            (5.0, 6.0),
            (0.2, 6.0),
            (0.2, 0.2),
            (6.0, 0.2),
        ]];

        let result = union::<Centi>(path1, path2, FillRule::default()).unwrap();
        let output: Vec<Vec<(f64, f64)>> = result.closed.into();
        assert_eq!(output, expected_output);
    }
}
