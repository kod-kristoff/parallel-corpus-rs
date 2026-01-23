use std::collections::HashMap;

/// Union-find data structure operations
pub trait UnionFindOperations<A> {
    /// What group does this belong to? */
    fn find(&mut self, x: A) -> A;
    /// Make these belong to the same group. */
    fn union(&mut self, x: A, y: A) -> A;
    /// Make these belong to the same group. */
    fn unions(&mut self, xs: &[A]);
}

#[derive(Debug, Clone, Default)]
pub struct UnionFind {
    rev: Vec<Option<usize>>,
}

impl UnionFindOperations<usize> for UnionFind {
    fn find(&mut self, x: usize) -> usize {
        while x >= self.rev.len() {
            self.rev.push(None);
        }
        if let Some(rev_x) = self.rev[x] {
            if rev_x != x {
                self.rev[x] = Some(self.find(rev_x));
            }
        } else {
            self.rev[x] = Some(x);
        }
        self.rev[x].unwrap()
    }
    fn union(&mut self, x: usize, y: usize) -> usize {
        let find_x = self.find(x);
        let find_y = self.find(y);
        if find_x != find_y {
            self.rev[find_y] = Some(find_x);
        }
        find_x
    }
    fn unions(&mut self, xs: &[usize]) {
        if !xs.is_empty() {
            let xs0 = xs[0];
            for x in &xs[1..] {
                self.union(xs0, *x);
            }
        }
    }
}
/// Make a union-find data structure
pub fn union_find() -> UnionFind {
    UnionFind::default()
}

pub struct Renumber<A> {
    bw: HashMap<String, usize>,
    fw: HashMap<usize, A>,
    i: usize,
    serialize: Box<dyn Fn(&A) -> String>,
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

    /// What number does (the serialization of) this element have?
    pub fn num(&mut self, a: A) -> usize {
        let s = (self.serialize)(&a);
        if let Some(n) = self.bw.get(&s) {
            *n
        } else {
            self.fw.insert(self.i, a);
            self.bw.insert(s, self.i);
            let ret = self.i;
            self.i += 1;
            ret
        }
    }
    /// What is the serialization of any element that has this number?
    pub fn un(&self, n: usize) -> Option<&A> {
        self.fw.get(&n)
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
pub fn renumber<A: std::fmt::Debug>() -> Renumber<A> {
    Renumber::new(Box::new(|a: &A| format!("{:?}", a)))
}

// /** Make a polymorphic union-find data structure

//   const uf = PolyUnionFind<string>(a => a.toLowerCase())
//   uf.repr('a') // => 0
//   uf.repr('A') // => 0
//   uf.find('a') // => 'a'
//   uf.find('A') // => 'a'
//   uf.find('a') == uf.find('b') // => false
//   uf.union('A', 'B')
//   uf.find('a') == uf.find('b') // => true
// */
// pub fn PolyUnionFind<A>(
//   serialize = (a: A) => JSON.stringify(a)
// ): UnionFind<A> & {repr: (a: A) => number} {
//   const {un, num} = Renumber(serialize)
//   const uf = UnionFind()
//   return {
//     /** What number does the group of this element have? */
//     repr: x => uf.find(num(x)),
//     find: x => un(uf.find(num(x))),
//     union: (x, y) => un(uf.union(num(x), num(y))),
//     unions: xs => uf.unions(xs.map(num)),
//   }
// }
pub struct PolyUnionFind<A> {
    re: Renumber<A>,
    uf: UnionFind,
}

impl<A> PolyUnionFind<A> {
    pub fn new(serialize: Box<dyn Fn(&A) -> String>) -> Self {
        Self {
            re: Renumber::new(serialize),
            uf: UnionFind::default(),
        }
    }

    pub fn repr(&mut self, x: A) -> usize {
        self.uf.find(self.re.num(x))
    }
}

impl<A> PolyUnionFind<A> {
    fn find(&mut self, x: A) -> &A {
        let num_x = self.re.num(x);
        match self.re.un(self.uf.find(num_x)) {
            Some(a) => a,
            None => unreachable!(),
        }
    }
    fn union(&mut self, x: A, y: A) -> &A {
        let num_x = self.re.num(x);
        let num_y = self.re.num(y);
        match self.re.un(self.uf.union(num_x, num_y)) {
            Some(a) => a,
            None => unreachable!(),
        }
    }
    fn unions(&mut self, xs: &[A]) {
        if xs.is_empty() {
            return;
        }
        todo!()
        // let num_xs_0 = self.re.num(xs[0]);
        // for x in &xs[1..] {
        //     let num_x = self.re.num(x);
        //     self.uf.union(num_xs_0, num_x);
        // }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn union_find() {
        let mut uf = UnionFind::default();
        assert_ne!(uf.find(10), uf.find(20));
        uf.union(10, 20);
        assert_eq!(uf.find(10), uf.find(20));
        uf.union(20, 30);
        assert_eq!(uf.find(10), uf.find(30));
        uf.unions(&[10, 40, 50]);
        assert_eq!(uf.find(20), uf.find(40));
        assert_eq!(uf.find(20), uf.find(50));
    }

    #[test]
    fn renumber_default() {
        let mut re = renumber();
        assert_eq!(re.num("foo"), 0);
        assert_eq!(re.num("bar"), 1);
        assert_eq!(re.num("foo"), 0);
        assert_eq!(re.un(0), Some(&"foo"));
        assert_eq!(re.un(1), Some(&"bar"));
        assert_eq!(re.un(2), None);
    }

    #[test]
    fn renumber_custom() {
        let mut re = Renumber::new(Box::new(|a: &&str| a.to_lowercase()));
        assert_eq!(re.num("foo"), 0);
        assert_eq!(re.num("FOO"), 0);
        assert_eq!(re.un(0), Some(&"foo"));
    }

    #[test]
    fn poly_find_union_custom() {
        let mut uf = PolyUnionFind::new(Box::new(|a: &&str| a.to_lowercase()));
        assert_eq!(uf.repr("a"), 0);
        assert_eq!(uf.repr("A"), 0);
        assert_eq!(uf.find("a"), &"a");
        assert_eq!(uf.find("A"), &"a");

        // let find_a = uf.find("a");
        // assert_ne!(find_a, uf.find("b"));
        // uf.union("a", "b");
        // assert_eq!(uf.find("a"), uf.find("b"));

        //   const uf = PolyUnionFind<string>(a => a.toLowerCase())
        //   uf.repr('a') // => 0
        //   uf.repr('A') // => 0
        //   uf.find('a') // => 'a'
        //   uf.find('A') // => 'a'
        //   uf.find('a') == uf.find('b') // => false
        //   uf.union('A', 'B')
        //   uf.find('a') == uf.find('b') // => true
    }
}
