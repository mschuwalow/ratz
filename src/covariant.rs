use super::family::*;
pub trait Covariant<'a>: Family<'a> {
    fn map<A: 'a, B: 'a, F: FnMut(A) -> B + 'a>(
        fa: Self::Member<A>,
        f: F,
    ) -> Self::Member<B>;
}

impl<'a> Covariant<'a> for OptionFamily {
    fn map<A: 'a, B: 'a, F: FnMut(A) -> B + 'a>(
        fa: Option<A>,
        f: F,
    ) -> Option<B> {
        fa.map(f)
    }
}
