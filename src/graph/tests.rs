use super::*;

#[test]
fn test_graph_init() {
    let g = Graph::init("w1 w2");
    let source = vec![
        Token::new("w1 ".to_string(), "s0".to_string()),
        Token::new("w2 ".to_string(), "s1".to_string()),
    ];
    //   const target = [{text: 'w1 ', id: 't0'}, {text: 'w2 ', id: 't1'}]
    //   const edges = edge_record([Edge(['s0', 't0'], []), Edge(['s1', 't1'], [])])
    //   g // => {source, target, edges}
    assert_eq!(g.source, source);
}
