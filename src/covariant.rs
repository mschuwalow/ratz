use super::hkt::*;
pub trait Covariant<'a>: Hkt<'a> {
    fn map<A: 'a, B: 'a, F: FnMut(A) -> B + 'a>(
        fa: Self::Member<A>,
        f: F,
    ) -> Self::Member<B>;
}
