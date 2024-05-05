use std::collections::HashMap;

#[cfg(test)]
mod tests;
// /** Union-find data structure operations */
// pub interface UnionFind<A> {
/// What group does this belong to?
//   find(x: A): A
/// Make these belong to the same group.
//   union(x: A, y: A): A
/// Make these belong to the same group.
//   unions(xs: A[]): void
// }

/// Make a union-find data structure

//   const uf = UnionFind()
//   uf.find(10) == uf.find(20) // => false
//   uf.union(10, 20)
//   uf.find(10) == uf.find(20) // => true
//   uf.union(20, 30)
//   uf.find(10) == uf.find(30) // => true
//   uf.unions([10, 40, 50])
//   uf.find(20) == uf.find(40) // => true
//   uf.find(20) == uf.find(50) // => true
///
#[derive(Debug, Default)]
pub struct UnionFind {
    rev: Vec<Option<usize>>,
}

impl UnionFind {
    // pub fn UnionFind(): UnionFind<number> {
    pub fn new() -> Self {
        Self::default()
    }
    /// What group does this belong to?
    pub fn find(&mut self, x: usize) -> usize {
        while x >= self.rev.len() {
            self.rev.push(None);
        }
        if self.rev[x].is_none() {
            self.rev[x] = Some(x);
        } else if self.rev[x] != Some(x) {
            self.rev[x] = Some(self.find(self.rev[x].unwrap()));
        }
        self.rev[x].unwrap()
    }
    /// Make these belong to the same group.
    pub fn union(&mut self, x: usize, y: usize) -> usize {
        let find_x = self.find(x);
        let find_y = self.find(y);
        if find_x != find_y {
            self.rev[find_y] = Some(find_x);
        }
        find_x
    }
    /// Make these belong to the same group.
    pub fn unions(&mut self, xs: &[usize]) {
        xs.iter().fold(xs[0], |xs_0, x| {
            self.union(xs_0, *x);
            xs_0
        });
    }
}
// /** Assign unique numbers to each distinct element

//   const {un, num} = Renumber()
//   num('foo') // => 0
//   num('bar') // => 1
//   num('foo') // => 0
//   un(0) // => 'foo'
//   un(1) // => 'bar'
//   un(2) // => undefined

//   const {un, num} = Renumber<string>(a => a.toLowerCase())
//   num('foo') // => 0
//   num('FOO') // => 0
//   un(0) // => 'foo'
// */
pub struct Renumber<A> {
    bw: HashMap<String, usize>,
    fw: HashMap<usize, A>,
    i: usize,
    serialize: Box<dyn Fn(&A) -> String>,
}

impl<A> Default for Renumber<A>
where
    A: ToString + 'static,
{
    fn default() -> Self {
        Self::new(Box::new(A::to_string))
    }
}

impl<A> Renumber<A> {
    pub fn new(serialize: Box<dyn Fn(&A) -> String>) -> Self {
        Self {
            bw: HashMap::new(),
            fw: HashMap::new(),
            i: 0,
            serialize,
        }
    }
    /// What number does (the serialization of) this element have? */
    pub fn num(&mut self, a: A) -> usize {
        let s = (self.serialize)(&a);
        if !self.bw.contains_key(&s) {
            let result = self.i;
            self.fw.insert(self.i, a);
            self.bw.insert(s, self.i);
            self.i += 1;
            return result;
        }
        self.bw[&s]
    }
    /// What is the serialization of any element that has this number? */
    pub fn un(&self, n: usize) -> Option<&A> {
        self.fw.get(&n)
    }
}

/// Make a polymorphic union-find data structure
//   const uf = PolyUnionFind<string>(a => a.toLowerCase())
//   uf.repr('a') // => 0
//   uf.repr('A') // => 0
//   uf.find('a') // => 'a'
//   uf.find('A') // => 'a'
//   uf.find('a') == uf.find('b') // => false
//   uf.union('A', 'B')
//   uf.find('a') == uf.find('b') // => true
///
pub struct PolyUnionFind<A> {
    renum: Renumber<A>,
    uf: UnionFind,
}

impl<A> PolyUnionFind<A> {
    pub fn new<F>(serialize: F) -> Self
    where
        F: Fn(&A) -> String + 'static,
    {
        Self {
            renum: Renumber::new(Box::new(serialize)),
            uf: UnionFind::default(),
        }
    }
    /// What number does the group of this element have?
    pub fn repr(&mut self, x: A) -> usize {
        let num_x = self.renum.num(x);
        self.uf.find(num_x)
    }
    /// What group does this belong to?
    pub fn find(&mut self, x: A) -> Option<&A> {
        let num_x = self.renum.num(x);
        let find_x = self.uf.find(num_x);
        self.renum.un(find_x)
    }
    /// Make these belong to the same group.
    pub fn union(&mut self, x: A, y: A) -> Option<&A> {
        let num_x = self.renum.num(x);
        let num_y = self.renum.num(y);
        let u_xy = self.uf.union(num_x, num_y);
        self.renum.un(u_xy)
    }
}
impl<A> PolyUnionFind<A>
where
    A: Clone,
{
    /// Make these belong to the same group.
    pub fn unions(&mut self, xs: &[A]) {
        if xs.is_empty() {
            return;
        }
        let num_0 = self.renum.num(xs[0].clone());
        xs.iter()
            .skip(1)
            .map(|x| self.renum.num(x.clone()))
            .for_each(|e| {
                self.uf.union(num_0, e);
            });
    }
}
