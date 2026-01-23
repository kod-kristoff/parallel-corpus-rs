#[derive(Debug)]
pub struct SourceTarget<A> {
    pub source: A,
    pub target: A,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Source,
    Target,
}

impl<A> SourceTarget<A> {
    pub fn get_side(&self, side: Side) -> &A {
        match side {
            Side::Source => &self.source,
            Side::Target => &self.target,
        }
    }
}

pub fn map_sides<A, B>(g: &SourceTarget<A>, f: impl Fn(&A, Side) -> B) -> SourceTarget<B> {
    SourceTarget {
        source: f(&g.source, Side::Source),
        target: f(&g.target, Side::Target),
    }
}
