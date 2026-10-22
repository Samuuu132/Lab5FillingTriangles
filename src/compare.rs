use std::collections::HashSet;
use crate::fragment::Fragment;

pub fn positions(fragments: &[Fragment]) -> HashSet<(i32, i32)> {
    fragments.iter().map(|f| (f.x, f.y)).collect()
}

pub fn differences(a: &[Fragment], b: &[Fragment]) -> (usize, usize) {
    let set_a = positions(a);
    let set_b = positions(b);
    let only_a = set_a.difference(&set_b).count();
    let only_b = set_b.difference(&set_a).count();
    (only_a, only_b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cases::{big_triangle, test_cases};
    use crate::triangle::{rasterize, Triangle, TriangleMode};

    #[test]
    fn both_algorithms_same_positions() {
        for case in test_cases() {
            let bary = rasterize(&case.triangles, TriangleMode::FilledBarycentric);
            let scan = rasterize(&case.triangles, TriangleMode::FilledScanline);
            assert_eq!(differences(&bary, &scan), (0, 0), "caso {}", case.name);
        }
    }

    #[test]
    fn reversed_vertices_same_triangle() {
        let t = big_triangle();
        let reversed = Triangle::new(t.c, t.b, t.a, t.color);

        for mode in [TriangleMode::FilledBarycentric, TriangleMode::FilledScanline] {
            let normal = rasterize(&[t], mode);
            let inverse = rasterize(&[reversed], mode);
            assert!(!normal.is_empty());
            assert_eq!(differences(&normal, &inverse), (0, 0));
        }
    }

    #[test]
    fn collinear_triangle_has_no_fragments() {
        let cases = test_cases();
        let case = cases.iter().find(|c| c.name == "colineal").unwrap();

        assert!(rasterize(&case.triangles, TriangleMode::FilledBarycentric).is_empty());
        assert!(rasterize(&case.triangles, TriangleMode::FilledScanline).is_empty());
    }
}
