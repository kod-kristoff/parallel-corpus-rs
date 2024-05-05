use super::*;

#[test]
fn test_graph_init() {
    let g = init("w1 w2");
    let source = vec![
        Token::new("w1 ".to_string(), "s0".to_string()),
        Token::new("w2 ".to_string(), "s1".to_string()),
    ];
    let target = vec![
        Token::new("w1 ".to_string(), "t0".to_string()),
        Token::new("w2 ".to_string(), "t1".to_string()),
    ];
    //   const target = [{text: 'w1 ', id: 't0'}, {text: 'w2 ', id: 't1'}]
    //   const edges = edge_record([Edge(['s0', 't0'], []), Edge(['s1', 't1'], [])])
    let edges = edge_record(vec![
        Edge::new(
            vec!["s0".to_string(), "t0".to_string()],
            vec![],
            false,
            None,
        ),
        Edge::new(
            vec!["s1".to_string(), "t1".to_string()],
            vec![],
            false,
            None,
        ),
    ]);
    assert_eq!(g.source, source);
    assert_eq!(g.target, target);
    assert_eq!(g.edges, edges);
}
