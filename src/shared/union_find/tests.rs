use super::*;

#[test]
fn test_union_find() {
    let mut uf = UnionFind::new();
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
fn test_renumber_default() {
    let mut renum = Renumber::default();

    assert_eq!(renum.num("foo"), 0);
    assert_eq!(renum.num("bar"), 1);
    assert_eq!(renum.num("foo"), 0);
    assert_eq!(renum.un(0), Some(&"foo"));
    assert_eq!(renum.un(1), Some(&"bar"));
    assert_eq!(renum.un(2), None);
}

#[test]
fn test_renumber_new() {
    let mut renum = Renumber::<String>::new(Box::new(|s| s.to_lowercase()));
    assert_eq!(renum.num("foo".to_string()), 0);
    assert_eq!(renum.num("FOO".to_string()), 0);
    assert_eq!(renum.un(0), Some(&"foo".to_string()));
}

#[test]
fn test_poly_union_find_str() {
    let mut uf = PolyUnionFind::new(Box::new(|s: &&str| s.to_lowercase()));
    assert_eq!(uf.repr("a"), 0);
    assert_eq!(uf.repr("A"), 0);
    assert_eq!(uf.find("a"), Some(&"a"));
    let find_a = uf.find("a").cloned();
    assert_ne!(find_a.as_ref(), uf.find("b"));
    uf.union("A", "B");
    let find_a = uf.find("a").cloned();
    assert_eq!(find_a.as_ref(), uf.find("b"));
}

#[test]
fn test_poly_unoin_find_string() {
    let mut uf = PolyUnionFind::<String>::new(|a: &String| a.to_lowercase());
    assert_eq!(uf.repr("a".to_string()), 0);
    assert_eq!(uf.repr("A".to_string()), 0);
    assert_eq!(uf.find("a".to_string()), Some(&"a".to_string()));
    assert_eq!(uf.find("A".to_string()), Some(&"a".to_string()));
    assert_ne!(
        uf.find("a".to_string()).cloned(),
        uf.find("b".to_string()).cloned()
    );
    uf.union("A".to_string(), "B".to_string());
    assert_eq!(
        uf.find("a".to_string()).cloned(),
        uf.find("b".to_string()).cloned()
    );
    //   uf.find('a') == uf.find('b') // => true
    uf.unions(&["A".to_string(), "c".to_string(), "d".to_string()]);
    assert_eq!(
        uf.find("A".to_string()).cloned(),
        uf.find("C".to_string()).cloned()
    );
    assert_eq!(
        uf.find("B".to_string()).cloned(),
        uf.find("d".to_string()).cloned()
    );
}
