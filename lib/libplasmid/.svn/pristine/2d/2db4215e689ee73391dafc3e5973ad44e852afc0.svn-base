use std::borrow::Borrow;

use crate::uni::RestrictionEnzyme;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Annotation {
    pub start: usize,
    pub needle: Option<usize>,
    pub end: usize,
    pub text: String,
}

impl Annotation {
    pub fn new<T>(start: usize, end: usize, needle: Option<usize>, text: T) -> Annotation
    where
        T: AsRef<str>,
    {
        Self {
            start,
            end,
            needle,
            text: text.as_ref().to_string(),
        }
    }

    pub fn new_from_restriction_enzyme<T>(
        start: usize,
        end: usize,
        needle: Option<usize>,
        enzyme: T,
    ) -> Annotation
    where
        T: Borrow<RestrictionEnzyme>,
    {
        Annotation::new(start, end, needle, &enzyme.borrow().name)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::RestrictionEnzymes;

    use super::Annotation;

    #[test]
    pub fn test_annotation_new() {
        let ann = Annotation::new(0, 10, Some(5), "foo");
        assert_eq!(ann.start, 0);
        assert_eq!(ann.needle, Some(5));
        assert_eq!(ann.end, 10);
        assert_eq!(ann.text, "foo");
    }

    #[test]
    pub fn test_annotation_new_from_restriction_enzyme() {
        let enzyme = RestrictionEnzymes
            .iter()
            .find(|r| r.name == "NdeI")
            .unwrap();
        let ann = Annotation::new_from_restriction_enzyme(0, 5, Some(3), enzyme);
        assert_eq!(ann.start, 0);
        assert_eq!(ann.needle, Some(3));
        assert_eq!(ann.end, 5);
        assert_eq!(ann.text, "NdeI");
    }
}
