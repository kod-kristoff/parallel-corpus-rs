use std::collections::HashMap;
use std::fmt;

// /** Parallel corpus as a graph */
use super::shared;
use super::shared::union_find;
use crate::token;
use crate::Token;
// import * as R from 'ramda'
// import * as Utils from '../Utils'
// import * as record from '../record'
// import {Token, Span} from './Token'
// import * as T from './Token'
// import {Lens, Store} from 'reactive-lens'

// import * as D from './Diff'

#[cfg(test)]
mod tests;

pub enum Side {
    Source,
    Target,
}

// pub let opposite = (s: Side): Side => (s === 'source' ? 'target' : 'source')

// pub let sides = ['source', 'target'] as Side[]

// pub let sidecase = <T>(side: Side, s: T, t: T): T => (side === 'source' ? s : t)
pub struct SourceTarget<A> {
    source: A,
    target: A,
}

impl<A: fmt::Debug> fmt::Debug for SourceTarget<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SourceTarget")
            .field("source", &self.source)
            .field("target", &self.target)
            .finish()
    }
}

pub fn map_sides<A, B>(g: &SourceTarget<A>, f: impl Fn(&A, Side) -> B) -> SourceTarget<B> {
    SourceTarget {
        source: f(&g.source, Side::Source),
        target: f(&g.target, Side::Target),
    }
    //   return {source: f(g.source, 'source'), target: f(g.target, 'target')}
}

// pub interface SourceTarget<A> {
//   readonly source: A
//   readonly target: A
// }

// pub interface Graph extends SourceTarget<Token[]> {
//   readonly edges: Edges
//   readonly comment?: string
// }
#[derive(Debug)]
pub struct Graph {
    source_target: SourceTarget<Vec<Token>>,
    edges: Edges,
    comment: Option<String>,
}

// pub type Edges = Record<string, Edge>
pub type Edges = HashMap<String, Edge>;

#[derive(Debug, PartialEq)]
pub struct Edge {
    /// a copy of the identifier used in the edges object of the graph
    id: String,
    /// these are ids to source and target tokens
    ids: Vec<String>,
    /// labels on this edge
    labels: Vec<String>,
    /// is this manually or automatically aligned
    manual: bool,
    comment: Option<String>,
}

impl Edge {
    pub fn new(
        mut ids: Vec<String>,
        labels: Vec<String>,
        manual: bool,
        comment: Option<String>,
    ) -> Edge {
        ids.sort();
        let labels_nub = shared::uniq(labels);
        Edge {
            id: format!("e-{}", ids.join("-")),
            ids,
            labels: labels_nub,
            manual,
            comment,
            // ...(comment && labels_nub.some(is_comment_label) ? {comment} : {}),
        }
    }
}

// pub fn merge_edges(...es: Edge[]) {
//   return Edge(
//     Utils.flatMap(es, e => e.ids),
//     Utils.flatMap(es, e => e.labels),
//     es.some(e => !!e.manual),
//     Utils.uniq(es.map(e => e.comment).filter(Boolean) as string[]).join('\n\n')
//   )
// }

// pub let zero_edge = merge_edges()

pub fn edge_record(es: Vec<Edge>) -> Edges {
    let mut out = Edges::new();
    es.into_iter().for_each(|e| {
        out.insert(e.id.clone(), e);
    });
    out
}

// /** Checks that the invariant of the graph holds

//   check_invariant(init('apa bepa cepa')) // => 'ok'

//   let g0 = init('apa')
//   let g = {...g0, edges: {'oops': g0.edges['e-s0-t0']}}
//   check_invariant(g) !== 'ok' // => true

// */
// pub fn check_invariant(g: Graph): 'ok' | {violation: string; g: Graph} {
//   try {
//     let tokens = g.source.concat(g.target)
//     {
//       let unique_id = Utils.unique_check<string>()
//       tokens.forEach(t => unique_id(t.id) || Utils.raise('Duplicate id: ' + t))
//       record.forEach(
//         g.edges,
//         e => unique_id(e.id) || Utils.raise('Duplicate id from edges: ' + e.id)
//       )
//     }
//     let check_tokens = (toks: string[]) => {
//       if (toks.length == 1) {
//         let t = toks[0]
//         t.match(/^\s*\S*\s*$/) || Utils.raise('Bad single token: ' + JSON.stringify(t))
//       } else {
//         toks.forEach(
//           (t, i) => t.match(/^\s*\S+\s+$/) || Utils.raise('Bad text token: ' + JSON.stringify(t))
//         )
//       }
//     }
//     check_tokens(target_texts(g))
//     check_tokens(source_texts(g))
//     record.forEach(
//       g.edges,
//       (e, id) =>
//         e.id === id || Utils.raise(`Edge key and id do not match: ${id} and ${Utils.show(e)}`)
//     )
//     record.forEach(
//       g.edges,
//       e =>
//         e.ids.length > 0 || Utils.raise(`Edge without any associated identifiers ${Utils.show(e)}`)
//     )
//     record.forEach(
//       g.edges,
//       e =>
//         !e.comment ||
//         e.labels.some(is_comment_label) ||
//         Utils.raise(`Edge with comment but no comment label: ${Utils.show(e)}`)
//     )
//     record.forEach(
//       g.edges,
//       e => R.equals(e, merge_edges(e)) || Utils.raise(`Edge not in normal form: ${Utils.show(e)}`)
//     )
//     {
//       let token_ids = new Set(tokens.map(t => t.id))
//       record.forEach(g.edges, e =>
//         e.ids.forEach(id => {
//           token_ids.has(id) || Utils.raise(`Edge ${Utils.show(e)} refers to unknown token ${id}`)
//         })
//       )
//     }
//     {
//       let token_count = Utils.count<string>()
//       record.forEach(g.edges, e => e.ids.forEach(id => token_count.inc(id)))
//       tokens.forEach(tok => {
//         let n = token_count.get(tok.id)
//         n == 1 || Utils.raise('Token not appearing exactly once in edge lists: ' + tok.id)
//       })
//     }
//     R.equals(g, align(g)) || Utils.raise('Graph not automatically aligned')
//     g.comment !== '' || Utils.raise('Graph comment must not be empty string')
//     record.forEach(
//       g.edges,
//       e => e.comment !== '' || Utils.raise('Edge comment must not be empty string')
//     )
//   } catch (e) {
//     // console.error(e)
//     // console.error(JSON.stringify(g, undefined, 2))
//     return {violation: e.toString(), g}
//   }
//   return 'ok'
// }

// /** Whether the object is a valid graph.

//   is_graph('foo') // => false
//   is_graph(init('apa bepa')) // => true
//  */
// pub fn is_graph(g: any): g is Graph {
//   return check_invariant(g) === 'ok'
// }

/// Makes spans from an original text by tokenizing it and assumes no changes
///
// # Examples
//   let g = init('w1 w2')
//   let source = [{text: 'w1 ', id: 's0'}, {text: 'w2 ', id: 's1'}]
//   let target = [{text: 'w1 ', id: 't0'}, {text: 'w2 ', id: 't1'}]
//   let edges = edge_record([Edge(['s0', 't0'], []), Edge(['s1', 't1'], [])])
//   g // => {source, target, edges}

// */
pub fn init(s: &str) -> Graph {
    //   return init_from(T.tokenize(s), manual)
    init_from(token::tokenize(s), false)
}

/// Makes a graph from tokens
pub fn init_from(tokens: Vec<String>, manual: bool) -> Graph {
    //   return align({
    //     source: T.identify(tokens, 's'),
    //     target: T.identify(tokens, 't'),
    //     edges: edge_record(tokens.map((_, i) => Edge(['s' + i, 't' + i], [], manual))),
    //   })
    let edges = edge_record(
        tokens
            .iter()
            .enumerate()
            .map(|(i, _)| Edge::new(vec![format!("s{i}"), format!("t{i}")], vec![], manual, None))
            .collect(),
    );
    align(Graph {
        source_target: SourceTarget {
            source: token::identify(tokens.clone(), "s"),
            target: token::identify(tokens, "t"),
        },
        edges,
        comment: None,
    })
}

// pub fn empty(g: Graph): boolean {
//   return !g.source.length || !g.target.length
// }

// /** Change or remove the graph-wide comment.

//   let g0 = init('apa bepa')
//   let g1 = set_comment(g0, 'foo')
//   g1.comment // => 'foo'
//   let g2 = set_comment(g1)
//   g2.comment // => undefined
//   let g3 = set_comment(g1, '')
//   g3.comment // => undefined
//  */
// pub fn set_comment(g: Graph, c?: string): Graph {
//   return c ? {...g, comment: c} : {source: g.source, target: g.target, edges: g.edges}
// }

// /** Clone a graph

//   let g = init('apa bepa')
//   let g2 = clone(g)                       // => g
//   g2 == g                                   // => false
//   g2.source == g.source                     // => false
//   g2.edges['e-s0-t0'] == g.edges['e-s0-t0'] // => false

//  */
// pub fn clone(graph: Graph): Graph {
//   return {
//     source: graph.source.map(x => x),
//     target: graph.target.map(x => x),
//     edges: record.map(graph.edges, x => ({...x})),
//     ...(graph.comment ? {comment: graph.comment} : {}),
//   }
// }

// /** Initialize a graph from unaligned tokens

//   from_unaligned({
//     source: [{text: 'apa ', labels: []}],
//     target: [{text: 'apa ', labels: []}]
//   }) // => init('apa')
//   equal(from_unaligned({
//     source: [{text: 'apa ', labels: []}],
//     target: [{text: 'bepa ', labels: []}]
//   }), set_target(init('apa'), 'bepa ')) // => true

// */
// pub fn from_unaligned(st: SourceTarget<{text: string; labels: string[]}[]>): Graph {
//   let edges: Record<string, Edge> = {}
//   let g = mapSides(st, (toks, side) =>
//     toks.map((tok, i) => {
//       let id = side[0] + i
//       let e = Edge([id], tok.labels, false)
//       edges[id] = e
//       return T.Token(tok.text, id)
//     })
//   )
//   return align({...g, edges})
// }

/// Map from token ids to edges

//   let g = init('w')
//   let e = Edge(['s0', 't0'], [])
//   let lhs = [...edge_map(g).entries()]
//   let rhs = [['s0', e], ['t0', e]]
//   lhs // => rhs

// */
pub fn edge_map(g: Graph) -> HashMap<String, Edge> {
//   return new Map(
//     Utils.flatten(record.traverse(g.edges, e => e.ids.map(id => [id, e] as [string, Edge])))
//   )
// }

// /** The edges from a set of ids

//   let g = init('w')
//   token_ids_to_edges(g, ['s0']) // => Object.values(g.edges)
//   token_ids_to_edges(g, ['t0']) // => Object.values(g.edges)
//   token_ids_to_edges(g, ['s0', 't0']) // => Object.values(g.edges)

// */
// pub fn token_ids_to_edges(g: Graph, ids: string[]): Edge[] {
//   let em = edge_map(g)
//   let out: Edge[] = []
//   let first = Utils.unique_check<string>()
//   ids.forEach(id => {
//     let e = em.get(id)
//     if (e && first(e.id)) {
//       out.push(e)
//     }
//   })
//   return out
// }

// pub fn token_ids_to_edge_ids(g: Graph, ids: string[]): string[] {
//   return token_ids_to_edges(g, ids).map(e => e.id)
// }

// /** Find tokens by token ids and split by source or target.

//   let g = init('a b c')
//   let source = [g.source[1], g.source[2]]
//   let target = [g.target[1], g.target[0]]
//   partition_ids(g)(['s1', 't1', 's2', 't0']) // => {source, target}

// */
// pub fn partition_ids(g: Graph): (ids: string[]) => SourceTarget<Token[]> {
//   let sm = source_map(g)
//   let tm = target_map(g)
//   return ids => {
//     let source = [] as Token[]
//     let target = [] as Token[]
//     ids.forEach(id => {
//       let s = sm.get(id)
//       if (s !== undefined) {
//         source.push(g.source[s])
//       }
//       let t = tm.get(id)
//       if (t !== undefined) {
//         target.push(g.target[t])
//       }
//     })
//     return {source, target}
//   }
// }

// pub type SidedIndex = {side: Side; index: number}

// /** Map from token identifiers to sided offsets

//   let g = init('a b c')
//   let m = token_map(g)
//   m.get('s0') // => {side: 'source', index: 0}
//   m.get('s1') // => {side: 'source', index: 1}
//   m.get('t0') // => {side: 'target', index: 0}

// */
// pub fn token_map(g: Graph): Map<string, SidedIndex> {
//   let m = mapSides(g, (tokens, side) =>
//     tokens.map((token, index) => [token.id, {side, index}] as [string, SidedIndex])
//   )
//   return new Map([...m.source, ...m.target])
// }

// /** Map from source identifiers to offsets

//   let g = init('a b c')
//   let m = source_map(g)
//   m.get('s0') // => 0
//   m.get('s1') // => 1
//   m.has('t0') // => false

// */
// pub fn source_map(g: SourceTarget<Token[]>): Map<string, number> {
//   return new Map(g.source.map((s, i) => [s.id, i] as [string, number]))
// }

// /** Map from target identifiers to offsets

//   let g = init('a b c')
//   let m = target_map(g)
//   m.get('t0') // => 0
//   m.get('t1') // => 1
//   m.has('s0') // => false

// */
// pub fn target_map(g: SourceTarget<Token[]>): Map<string, number> {
//   return new Map(g.target.map((t, i) => [t.id, i] as [string, number]))
// }

// /** The edge at a position (in the target text)

//   let g = init('apa bepa cepa')
//   edge_at(g, 1) // => Edge(['s1', 't1'], [])

// */
// pub fn edge_at(g: Graph, index: number, side: Side = 'target'): Edge {
//   let token_id = g[side][index].id
//   return edge_map(g).get(token_id) || Utils.raise('Out of bounds: ' + JSON.stringify({g, index}))
// }

// /** The related ids at a position (in the target text)

//   let g = init('apa bepa cepa')
//   related(g, 1) // => ['s1', 't1']

// */
// pub fn related(g: Graph, index: number): string[] {
//   return edge_at(g, index).ids
// }

// /** The text in the target

//   target_text(init('apa bepa cepa ')) // => 'apa bepa cepa '

// */
// pub fn target_text(g: SourceTarget<T.Text[]>): string {
//   return T.text(g.target)
// }

// /** The text in the source

//   source_text(init('apa bepa cepa ')) // => 'apa bepa cepa '

// */
// pub fn source_text(g: SourceTarget<T.Text[]>): string {
//   return T.text(g.source)
// }

// /** The texts in the target

//   target_texts(init('apa bepa cepa ')) // => ['apa ', 'bepa ', 'cepa ']

// */
// pub fn target_texts(g: SourceTarget<T.Text[]>): string[] {
//   return T.texts(g.target)
// }

// /** The texts in the source

//   source_texts(init('apa bepa cepa ')) // => ['apa ', 'bepa ', 'cepa ']

// */
// pub fn source_texts(g: SourceTarget<T.Text[]>): string[] {
//   return T.texts(g.source)
// }

// /** The next free unique id

//   next_id(init('apa')) // => 1

// */
// pub fn next_id(g: Graph): number {
//   return Utils.next_id([...g.target.map(t => t.id), ...g.source.map(t => t.id)])
// }

// /** Replace the text at some position, merging the spans it touches upon.

//   let show = (g: Graph) => g.target.map(t => t.text)
//   let ids = (g: Graph) => g.target.map(t => t.id).join(' ')
//   let g = init('test graph hello')
//   show(g) // => ['test ', 'graph ', 'hello ']
//   show(unaligned_modify(g, 0, 0, 'new')) // => ['newtest ', 'graph ', 'hello ']
//   show(unaligned_modify(g, 0, 1, 'new')) // => ['newest ', 'graph ', 'hello ']
//   show(unaligned_modify(g, 0, 5, 'new ')) // => ['new ', 'graph ', 'hello ']
//   show(unaligned_modify(g, 0, 5, 'new')) // => ['newgraph ', 'hello ']
//   show(unaligned_modify(g, 5, 5, ' ')) // => ['test ', ' graph ', 'hello ']
//   show(unaligned_modify(g, 5, 6, ' ')) // => ['test ', ' raph ', 'hello ']
//   show(unaligned_modify(g, 0, 15, '_')) // => ['_o ']
//   show(unaligned_modify(g, 0, 16, '_')) // => ['_ ']
//   show(unaligned_modify(g, 0, 17, '_')) // => ['_ ']
//   show(unaligned_modify(g, 16, 16, ' !')) // => ['test ', 'graph ', 'hello ', '! ']

// Indexes are character offsets (use CodeMirror's doc.posFromIndex and doc.indexFromPos to convert) */
// pub fn unaligned_modify(
//   g: Graph,
//   from: number,
//   to: number,
//   text: string,
//   side: Side = 'target'
// ): Graph {
//   let tokens = get_side_texts(g, side)
//   let {token: from_token, offset: from_ix} = T.token_at(tokens, from)
//   let pre = (tokens[from_token] || '').slice(0, from_ix)
//   if (to === get_side_text(g, side).length) {
//     return unaligned_modify_tokens(g, from_token, g[side].length, pre + text, side)
//   } else {
//     let {token: to_token, offset: to_ix} = T.token_at(tokens, to)
//     let post = (tokens[to_token] || '').slice(to_ix)
//     return unaligned_modify_tokens(g, from_token, to_token + 1, pre + text + post, side)
//   }
// }

// /** Replace the text at some position, merging the spans it touches upon.

//   let show = (g: Graph) => g.target.map(t => t.text)
//   let ids = (g: Graph) => g.target.map(t => t.id).join(' ')
//   let g = init('test graph hello')
//   show(g) // => ['test ', 'graph ', 'hello ']
//   show(unaligned_modify_tokens(g, 0, 0, 'this '))     // => ['this ', 'test ', 'graph ', 'hello ']
//   show(unaligned_modify_tokens(g, 0, 1, 'this '))     // => ['this ', 'graph ', 'hello ']
//   show(unaligned_modify_tokens(g, 0, 1, '  white '))  // => ['  white ', 'graph ', 'hello ']
//   show(unaligned_modify_tokens(g, 0, 1, 'this'))      // => ['thisgraph ', 'hello ']
//   show(unaligned_modify_tokens(g, 1, 2, 'graph'))     // => ['test ', 'graphhello ']
//   show(unaligned_modify_tokens(g, 1, 2, ' graph '))   // => ['test ', ' graph ', 'hello ']
//   show(unaligned_modify_tokens(g, 0, 1, 'for this ')) // => ['for ', 'this ', 'graph ', 'hello ']
//   show(unaligned_modify_tokens(g, 0, 2, '')) // => ['hello ']
//   show(unaligned_modify_tokens(g, 0, 2, '  ')) // => ['  hello ']
//   show(unaligned_modify_tokens(g, 1, 3, '  ')) // => ['test   ']
//   show(unaligned_modify_tokens(g, 3, 3, ' !')) // => ['test ', 'graph ', 'hello  ', '! ']
//   show(unaligned_modify_tokens(init('a '), 0, 1, ' ')) // => [' ']
//   ids(g) // => 't0 t1 t2'
//   ids(unaligned_modify_tokens(g, 0, 0, 'this '))     // => 't3 t0 t1 t2'
//   ids(unaligned_modify_tokens(g, 0, 1, 'this '))     // => 't3 t1 t2'
//   ids(unaligned_modify_tokens(g, 0, 1, 'this'))      // => 't3 t2'
//   let showS = (g: Graph) => g.source.map(t => t.text)
//   let idsS = (g: Graph) => g.source.map(t => t.id).join(' ')
//   showS(unaligned_modify_tokens(g, 0, 0, 'this ', 'source')) // => ['this ', 'test ', 'graph ', 'hello ']
//   idsS(unaligned_modify_tokens(g, 0, 0, 'this ', 'source'))  // => 's3 s0 s1 s2'

// Indexes are token offsets */
// pub fn unaligned_modify_tokens(
//   g: Graph,
//   from: number,
//   to: number,
//   text: string,
//   side: Side = 'target'
// ): Graph {
//   if (from < 0 || to < 0 || from > g[side].length || to > g[side].length || from > to) {
//     throw new Error('Invalid coordinates ' + Utils.show({g, from, to, text}))
//   }
//   if (text.match(/^\s+$/)) {
//     // replacement text is only whitespace: need to find some token to put it on
//     if (from > 0) {
//       return unaligned_modify_tokens(g, from - 1, to, g[side][from - 1].text + text, side)
//     } else if (to < g[side].length) {
//       return unaligned_modify_tokens(g, from, to + 1, text + g[side][to].text, side)
//     } else {
//       // console.warn('Introducing whitespace into empty graph')
//     }
//   }
//   if (text.match(/\S$/) && to < g[side].length) {
//     // if replacement text does not end with whitespace, grab the next word as well
//     return unaligned_modify_tokens(g, from, to + 1, text + g[side][to].text, side)
//   }

//   if (from > 0 && from == g[side].length && to === g[side].length) {
//     // we're adding a word at the end but the last token might not end in whitespace:
//     // glue them together

//     return unaligned_modify_tokens(g, from - 1, to, g[side][from - 1].text + text, side)
//   }

//   let id_offset = next_id(g)
//   let tokens = T.tokenize(text).map((t, i) => Token(t, side[0] + (id_offset + i)))
//   let [new_tokens, removed] = Utils.splice(g[side], from, to - from, ...tokens)
//   let ids_removed = new Set(removed.map(t => t.id))
//   let new_edge_ids = new Set<string>(tokens.map(t => t.id))
//   let new_edge_labels = new Set<string>()
//   let new_edge_manual = false
//   let edges = record.filter(g.edges, e => {
//     if (e.ids.some(id => ids_removed.has(id))) {
//       e.ids.forEach(id => ids_removed.has(id) || new_edge_ids.add(id))
//       e.labels.forEach(lbl => new_edge_labels.add(lbl))
//       new_edge_manual = new_edge_manual || e.manual === true
//       return false
//     } else {
//       return true
//     }
//   })
//   if (new_edge_ids.size > 0) {
//     let e = Edge([...new_edge_ids], [...new_edge_labels], new_edge_manual)
//     edges[e.id] = e
//   }
//   return {...g, [side]: new_tokens, edges}
// }

// pub fn modify(
//   g: Graph,
//   from: number,
//   to: number,
//   text: string,
//   side: Side = 'target'
// ): Graph {
//   return align(unaligned_modify(g, from, to, text, side))
// }

// pub fn modify_tokens(
//   g: Graph,
//   from: number,
//   to: number,
//   text: string,
//   side: Side = 'target'
// ): Graph {
//   return align(unaligned_modify_tokens(g, from, to, text, side))
// }

// /** Moves a slice of the target tokens and puts it at a new destination.

//   target_text(unaligned_rearrange(init('apa bepa cepa depa'), 1, 2, 0)) // => 'bepa cepa apa depa '

// Indexes are token offsets
// */
// pub fn unaligned_rearrange(g: Graph, begin: number, end: number, dest: number): Graph {
//   let em = edge_map(g)
//   let edge_ids_to_update = new Set(
//     g.target.slice(begin, end + 1).map(t => (em.get(t.id) as Edge).id)
//   )
//   let new_edges = {} as Record<string, Edge>
//   edge_ids_to_update.forEach(id => {
//     new_edges[id] = merge_edges(g.edges[id], Edge([], [], true))
//   })
//   return {
//     ...g,
//     source: g.source,
//     target: Utils.rearrange(g.target, begin, end, dest),
//     edges: {...g.edges, ...new_edges},
//   }
// }

// pub fn rearrange(g: Graph, begin: number, end: number, dest: number): Graph {
//   return align(unaligned_rearrange(g, begin, end, dest))
// }

// pub fn unaligned_set_side(g: Graph, side: Side, text: string): Graph {
//   let text0 = get_side_text(g, side)
//   let {from, to} = Utils.edit_range(text0, text)
//   let new_text = text.slice(from, text.length - (text0.length - to))
//   return unaligned_modify(g, from, to, new_text, side)
// }

// /**

//   target_text(set_target(init('apa bepa'), 'aupa bpa ')) // => 'aupa bpa '
//   target_text(set_target(init('fz'), 'bar ')) // => 'bar '
//   target_text(set_target(init('foo'), 'bar ')) // => 'bar '
//   target_text(set_target(init('fooz'), 'bar ')) // => 'bar '
//   target_text(set_target(init('a'), 'a ')) // => 'a '
//   target_text(set_target(init('a'), '  ')) // => '  '

// */
// pub fn set_target(g: Graph, text: string): Graph {
//   return align(unaligned_set_side(g, 'target', text))
// }

// pub fn set_source(g: Graph, text: string): Graph {
//   return align(unaligned_set_side(g, 'source', text))
// }

// pub fn set_side(g: Graph, side: Side, text: string): Graph {
//   return sidecase(side, set_source, set_target)(g, text)
// }

// pub fn get_side_text(g: Graph, side: Side): string {
//   return T.text(g[side])
// }

// pub fn get_side_texts(g: Graph, side: Side): string[] {
//   return T.texts(g[side])
// }

// /** Invert the graph: swap source and target, without aligning */
// pub fn unaligned_invert(g: Graph): Graph {
//   return {...g, source: g.target, target: g.source}
// }

// /** Invert the graph: swap source and target.

// Note that this is not stable, ie not involutive since texts get automatically
// realigned. This can make labels get transferred between groups.
// */
// pub fn invert(g: Graph): Graph {
//   return align(unaligned_invert(g))
// }

// /** Revert at an edge id */
// pub fn unaligned_revert(g: Graph, edge_ids: string[]): Graph {
//   let edge_set = new Set(edge_ids)
//   let diff = calculate_dnd_diff(g)
//   let supply = next_id(g)
//   let edges = record.filter(g.edges, (_, id) => !edge_set.has(id))
//   let reverted = Utils.flatMap(
//     diff,
//     D.dnd_match({
//       Dragged(d) {
//         if (edge_set.has(d.id)) {
//           let s = d.source
//           let t = {...d.source, id: 't' + supply++}
//           let e = Edge([s.id, t.id], [])
//           edges[e.id] = e
//           return [D.Dragged(s, e.id, false), D.Dropped(t, e.id, false)]
//         } else {
//           return [d]
//         }
//       },
//       Dropped(d) {
//         if (edge_set.has(d.id)) {
//           return []
//         } else {
//           return [d]
//         }
//       },
//     })
//   )
//   return {...g, ...from_dnd_diff(reverted, edges)}
// }

// /** Revert at an edge id */
// pub fn revert(g: Graph, edge_ids: string[]): Graph {
//   return align(unaligned_revert(g, edge_ids))
// }

// /** Connect edges by ids */
// pub fn connect(g: Graph, edge_ids: string[]): Graph {
//   let edges = record.filter(g.edges, (e, _) => !edge_ids.some(id => id == e.id))
//   let es = record.traverse(
//     record.filter(g.edges, (e, _) => edge_ids.some(id => id == e.id)),
//     e => e
//   )
//   let edge = merge_edges(...es, Edge([], [], true))
//   edges[edge.id] = edge
//   return align({...g, edges})
// }

// /** Disconnect a source or target id */
// pub fn disconnect(g: Graph, ids: string[]): Graph {
//   if (ids.length == 0) {
//     return align(g)
//   }
//   let id = ids[0]
//   let em = edge_map(g)
//   let edge = em.get(id)
//   if (edge) {
//     let edge_without = Edge(edge.ids.filter(i => i != id), edge.labels, true, edge.comment)
//     let edge_with = Edge([id], [], true, edge.comment)
//     let edges = record.filter(g.edges, (_, id) => id != edge.id)
//     edges[edge_with.id] = edge_with
//     if (edge_without.ids.length > 0) {
//       edges[edge_without.id] = edge_without
//     }
//     return disconnect({...g, edges}, ids.slice(1))
//   } else {
//     Utils.stderr({id, ids, g})
//     return Utils.raise('Trying to disconnect unidentifiable token')
//   }
// }

// /** Get the index of the first token of an edge.

//   let g = init('apa bepa cepa ')
//   let e = g.edges['e-s1-t1']
//   edge_first_index(g, e, 'source') // => 1

//  */
// pub fn edge_first_index(g: Graph, edge: Edge, side: Side): number | undefined {
//   return edge.ids
//     .map(id => token_map(g).get(id) as SidedIndex)
//     .filter(si => si.side == side)
//     .map(si => si.index)
//     .shift()
// }

// /** Group edges into groups of consecutive tokens.

//   let g = init('apa bepa cepa depa ')
//   let es = [g.edges['e-s0-t0'], g.edges['e-s1-t1'], g.edges['e-s3-t3']]
//   group_consecutive(g, es, 'source') // => [[es[0], es[1]], [es[2]]]

//  */
// pub fn group_consecutive(g: Graph, edges: Edge[], side: Side) {
//   return Utils.group_contiguous(edges, e => {
//     let i = edge_first_index(g, e, side)
//     return i !== undefined ? i : -1
//   })
// }

// interface CharIdPair {
//   char: string
//   id?: string
// }

// /**

//   [
//     {char: ' ', id: undefined},
//     {char: 'a', id: 'id'},
//     {char: 'b', id: 'id'},
//     {char: ' ', id: undefined}
//   ] // => to_char_ids(Token(' ab ', 'id'))

// */
// fn to_char_ids(token: Token): CharIdPair[] {
//   return Utils.str_map(token.text, char => ({char, id: char === ' ' ? undefined : token.id}))
// }

// /** Create edges automatically between similar sequences of tokens.

//   let g0 = {...init('a bc d')}
//   let g = unaligned_set_side(g0, 'target', 'ab c d')
//   Object.values(align(g).edges).length // => 2
// */
pub fn align(g: Graph) -> Graph {
    // Use a union-find to group characters into edges.
    let uf = union_find::PolyUnionFind::<String>::new(|u| u.clone());
    //   let em = Utils.chain(edge_map(g), m => (id: string): Edge =>
    //     m.get(id) || Utils.raise(`Token id ${id} not in edge map`)
    //   )

    //   {
    // Character by character, what was deleted and inserted?
    // let chars = map_sides(&g.source_target, |tokens, _| tokens.iter().filter(|token| ));
    //     let chars = mapSides(g, tokens =>
    //       Utils.flatMap(tokens.filter(token => !em(token.id).manual), to_char_ids)
    //     )
    //     let char_diff = Utils.hdiff(chars.source, chars.target, u => u.char, u => u.char)

    //     // For any unchanged character, unify its source and target tokens.
    //     // If source is "a bc" and target is "ab c", all characters will be unified to the same group.
    //     // The union-find operates over token ids, so an edge is represented by a "root" token id.
    //     char_diff.forEach(c => {
    //       if (c.change == 0) {
    //         // these undefined makes the alignment skip spaces.
    //         // they originate from to_char_ids
    //         if (c.a.id !== undefined && c.b.id !== undefined) {
    //           uf.union(c.a.id, c.b.id)
    //         }
    //       }
    //     })
    //   }

    //   // Use manual edges as they are.
    //   let proto_edges = record.filter(g.edges, e => !!e.manual)

    //   let first = Utils.unique_check<string>()

    //   mapSides(g, (tokens, side) =>
    //     tokens.forEach(token => {
    //       let e_repr = em(token.id)
    //       // Skip manual edges, they have already been added.
    //       if (!e_repr.manual) {
    //         // Use the labels from the old edge.
    //         let labels = first(e_repr.id) ? e_repr.labels : []
    //         // New edges are temporarily keyed by the "root" token id.
    //         // Merge a single-token edge into the edge that has the same "root" token.
    //         // Or add as a new edge if there is no such edge yet.
    //         let e_token = Edge([token.id], labels, false, e_repr.comment)
    //         record.modify(proto_edges, uf.find(token.id), zero_edge, e => merge_edges(e, e_token))
    //       }
    //     })
    //   )

    //   // Re-key edges.
    //   let edges = edge_record(record.traverse(proto_edges, e => e))

    // Graph { edges, ..g }
    g
}

// interface ScoreDiffPair {
//   score: number
//   // A reversed list of the way back (Instead of letructing it from back links)
//   diff: Utils.LazySnocList<D.Diff>
// }

// /** Calculate the (graphView) diff

// What we do here is try to find a diff using dragged and dropped looking only
// at the edge ids. This is different from finding a normal diff edit script
// over an alphabet because the edge ids may be used at several discontinuous
// locations that all should be close to each other. This was done using
// the diff algorithm before but the results were subpar, see #32

//   let expect: D.Diff[] = [
//     {
//       edit: 'Dragged',
//       source: {text: 'apa ', id: 's0'},
//       id: "e-s0-t0",
//       manual: true
//     },
//     {
//       edit: 'Edited',
//       source: [{text: 'bepa ', id: 's1'}],
//       target: [{text: 'bepa ', id: 't1'}],
//       id: "e-s1-t1",
//       manual: true
//     },
//     {
//       edit: 'Edited',
//       source: [{text: 'cepa ', id: 's2'}],
//       target: [{text: 'cepa ', id: 't2'}],
//       id: "e-s2-t2",
//       manual: true
//     },
//     {
//       edit: 'Dropped',
//       target: {text: 'apa ', id: 't0'},
//       id: "e-s0-t0",
//       manual: true
//     }
//   ]
//   let g = calculate_diff(rearrange(init('apa bepa cepa ', true), 1, 2, 0))
//   g // => expect

//   let expect: D.Diff[] = [
//     {
//       edit: 'Edited',
//       source: [{text: 'apa ', id: 's0'}],
//       target: [{text: 'apa ', id: 't0'}],
//       id: "e-s0-t0",
//       manual: true
//     }
//     {
//       edit: 'Edited',
//       source: [{text: 'bepa ', id: 's1'}],
//       target: [
//         {text: 'depa ', id: 't3'},
//         {text: 'epa ', id: 't4'}
//       ],
//       id: "e-s1-t3-t4",
//       manual: true
//     },
//     {
//       edit: 'Edited',
//       source: [{text: 'cepa ', id: 's2'}],
//       target: [{text: 'cepa ', id: 't2'}],
//       id: "e-s2-t2",
//       manual: true
//     }
//   ]
//   let g = calculate_diff(modify_tokens(init('apa bepa cepa ', true), 1, 2, 'depa epa '))
//   g // => expect

// */
// pub fn calculate_diff(
//   g: Graph,
//   order_changing_label: (s: string) => boolean = () => false
// ): D.Diff[] {
//   let m = edge_map(g)
//   let lookup = (tok: Token) => m.get(tok.id) as Edge

//   let I = g.source.length
//   let J = g.target.length

//   let OPT: ScoreDiffPair[][] = new Array(I + 1)
//     .fill({})
//     .map(i => new Array(J + 1).fill({score: 0, diff: null}))

//   fn opt(i: number, j: number) {
//     if (i < 0 && j < 0) {
//       return {score: 0, diff: null}
//     } else {
//       return OPT[i + 1][j + 1]
//     }
//   }

//   for (let i = -1; i < I; ++i) {
//     for (let j = -1; j < J; ++j) {
//       let cands: ScoreDiffPair[] = []
//       let same = (ii: number, jj: number) =>
//         ii >= 0 && jj >= 0 && lookup(g.source[ii]).id === lookup(g.target[jj]).id
//       if (i >= 0 && j >= 0 && same(i, j)) {
//         let ii = i
//         let jj = j
//         while (same(--ii, j));
//         while (same(i, --jj));
//         let edge = lookup(g.source[i])
//         let {score, diff} = opt(ii, jj)
//         let factor = 1
//         if (edge.manual) {
//           factor *= 0.01
//         }
//         if (edge.labels.some(order_changing_label)) {
//           factor *= 0.0001
//         }
//         cands.push({
//           score: score + factor * (i - ii + (j - jj)),
//           diff: Utils.snoc(
//             diff,
//             D.Edited(
//               g.source.slice(ii + 1, i + 1),
//               g.target.slice(jj + 1, j + 1),
//               edge.id,
//               !!edge.manual
//             )
//           ),
//         })
//       }
//       if (j >= 0) {
//         let {score, diff} = opt(i, j - 1)
//         let edge = lookup(g.target[j])
//         cands.push({score, diff: Utils.snoc(diff, D.Dropped(g.target[j], edge.id, !!edge.manual))})
//       }
//       if (i >= 0) {
//         let {score, diff} = opt(i - 1, j)
//         let edge = lookup(g.source[i])
//         cands.push({score, diff: Utils.snoc(diff, D.Dragged(g.source[i], edge.id, !!edge.manual))})
//       }
//       OPT[i + 1][j + 1] = R.sortBy(x => -x.score, cands)[0]
//     }
//   }

//   let {score, diff} = opt(I - 1, J - 1)
//   let arr = Utils.snocsToArray(diff)
//   return arr
// }

// /**

//   let diff: D.Diff[] = [
//     {
//       edit: 'Edited',
//       source: [{text: 'a ', id: 's0'}],
//       target: [{text: 'b ', id: 't0'}],
//       id: 'e0',
//       manual: true
//     },
//     {
//       edit: 'Edited',
//       source: [{text: 'c ', id: 's1'}],
//       target: [
//         {text: 'd ', id: 't3'},
//         {text: 'e ', id: 't4'}
//       ],
//       id: 'e1',
//       manual: true
//     }
//   ]
//   let expected: D.Diff[] = [
//     {edit: 'Dragged', source: {text: 'a ', id: 's0'}, id: 'e0', manual: true},
//     {edit: 'Dropped', target: {text: 'b ', id: 't0'}, id: 'e0', manual: true},
//     {edit: 'Dragged', source: {text: 'c ', id: 's1'}, id: 'e1', manual: true},
//     {edit: 'Dropped', target: {text: 'd ', id: 't3'}, id: 'e1', manual: true},
//     {edit: 'Dropped', target: {text: 'e ', id: 't4'}, id: 'e1', manual: true}
//   ]
//   split_up_edits(diff) // => expected
//   split_up_edits(diff, id => id == 'e0') // => [...expected.slice(0,2), diff[1]]
//   split_up_edits(diff, id => id == 'e1') // => [diff[0], ...expected.slice(2)]
//   split_up_edits(diff, _ => false) // => diff

// */
// pub fn split_up_edits(ds: D.Diff[], audit = (edge_id: string) => true): D.Diff[] {
//   return Utils.flatMap<D.Diff, D.Diff>(ds, d => {
//     if (d.edit == 'Edited' && audit(d.id)) {
//       return [
//         ...d.source.map(t => D.Dragged(t, d.id, d.manual)),
//         ...d.target.map(t => D.Dropped(t, d.id, d.manual)),
//       ]
//     } else {
//       return [d]
//     }
//   })
// }

// pub fn calculate_dnd_diff(
//   g: Graph,
//   order_changing_label: (s: string) => boolean = () => false
// ): (D.Dragged | D.Dropped)[] {
//   return split_up_edits(calculate_diff(g, order_changing_label)) as any
// }

// pub fn from_dnd_diff(
//   diff: (D.Dragged | D.Dropped)[],
//   edges0: Record<string, Edge>
// ): Graph {
//   let source = [] as Token[]
//   let target = [] as Token[]
//   let edges = R.clone(edges0)
//   diff.forEach(d =>
//     record.modify(edges, d.id, zero_edge, e => merge_edges(e, Edge([], [], d.manual)))
//   )
//   diff.forEach(
//     D.dnd_match({
//       Dragged: d => source.push(d.source),
//       Dropped: d => target.push(d.target),
//     })
//   )
//   return {source, target, edges}
// }

// /**

//   let g = modify_tokens(init('apa bepa cepa '), 1, 2, 'depa epa ')
//   let diff = calculate_diff(g)
//   let g2 = diff_to_graph(diff, g.edges)
//   g2 // => g

// */
// pub fn diff_to_graph(diff: D.Diff[], edges: Record<string, Edge>): Graph {
//   return align(from_dnd_diff(split_up_edits(diff) as any, edges))
// }

// /** Gets the sentence in the target text around some offset, without thinking about edits */
// pub fn target_sentence(g: Graph, i: number): Span {
//   return T.sentence(target_texts(g), i)
// }

// pub type Subspan = {source: Span; target: Span}

// pub fn subspan_merge(ss: Subspan[]) {
//   let {source, target} = ss[0]
//   ss.forEach(s => {
//     source = T.span_merge(source, s.source)
//     target = T.span_merge(target, s.target)
//   })
//   return {source, target}
// }

// pub fn subspan_to_indicies(subspan: Subspan): SidedIndex[] {
//   let span_to_indicies = (side: Side) => [
//     {side, index: subspan[side].begin},
//     {side, index: subspan[side].end},
//   ]
//   return [...span_to_indicies('source'), ...span_to_indicies('target')]
// }

// /** Gets the sentence in the target text around some offset(s)

//   let g = init('apa bepa . Cepa depa . epa ', true)
//   sentences(g, 0) // => {source: {begin: 0, end: 2}, target: {begin: 0, end: 2}}
//   sentences(g, 1) // => {source: {begin: 0, end: 2}, target: {begin: 0, end: 2}}
//   sentences(g, 2) // => {source: {begin: 0, end: 2}, target: {begin: 0, end: 2}}
//   sentences(g, 3) // => {source: {begin: 3, end: 5}, target: {begin: 3, end: 5}}
//   let g2 = modify_tokens(g, 1, 4, 'uff ! Hepp plepp ')
//   target_text(g2) // => 'apa uff ! Hepp plepp depa . epa '
//   sentences(g2, 0) // => {source: {begin: 0, end: 5}, target: {begin: 0, end: 6}}
//   sentences(g2, 1) // => {source: {begin: 0, end: 5}, target: {begin: 0, end: 6}}
//   sentences(g2, 2) // => {source: {begin: 0, end: 5}, target: {begin: 0, end: 6}}
//   sentences(g2, 3) // => {source: {begin: 0, end: 5}, target: {begin: 0, end: 6}}
//   let g3 = modify_tokens(g, 6, 7, '')
//   target_text(g3) // => 'apa bepa . Cepa depa . '
//   sentences(g3, 4) // => {source: {begin: 3, end: 6}, target: {begin: 3, end: 5}}
//   sentences(g3, 5) // => {source: {begin: 3, end: 6}, target: {begin: 3, end: 5}}

// */
// pub fn sentences(g: Graph, target_index: number): Subspan {
//   return sentences_around(g, [{side: 'target', index: target_index}])
// }

// /** Gets the sentences around some indicies */
// pub fn sentences_around(g: Graph, indicies: SidedIndex[]): Subspan {
//   let starts = Utils.PolyUnionFind<SidedIndex>()
//   let bounds = mapSides(g, (tokens, side) => {
//     let bs = T.sentence_starts(T.texts(tokens))
//     bs.forEach((start, index) => {
//       starts.union({side, index}, {side, index: start})
//     })
//     return bs
//   })
//   let m = token_map(g)
//   record.forEach(g.edges, e => {
//     let ids = e.ids.map(id => m.get(id) as SidedIndex)
//     starts.unions(ids)
//   })
//   starts.unions(indicies)

//   let main = indicies[0]

//   // grr-ish: we want to get the "minimal" representative now, but have to loop over
//   // all positions to check.
//   let em = edge_map(g)
//   let main_repr = starts.repr(main)
//   return mapSides(g, (tokens, side) => {
//     // If sentence starts or ends with only removed tokens we slurp these straggler tokens:
//     fn pad_missing(d: number, index: number): number {
//       if (index < 0) {
//         return 0
//       }
//       if (index >= tokens.length) {
//         return tokens.length - 1
//       }
//       let adjacent = tokens[index + d]
//       if (!adjacent) {
//         return index
//       }
//       let edge = em.get(adjacent.id)
//       if (!edge) {
//         return index
//       }
//       let all_on_this_side = edge.ids.every(id => {
//         let token = m.get(id)
//         return token ? token.side == side : false
//       })
//       if (!all_on_this_side) {
//         return index
//       }
//       return pad_missing(d, index + d)
//     }
//     return {
//       begin: pad_missing(
//         -1,
//         tokens.findIndex((_, index) => starts.repr({side, index}) == main_repr)
//       ),
//       end: pad_missing(
//         1,
//         Utils.findLastIndex(tokens, (_, index) => starts.repr({side, index}) == main_repr)
//       ),
//     }
//   })
// }

// /** All sentences in a text starting from an offset in the target text. */
// pub fn all_sentences(g: Graph, begin: number = 0): Subspan[] {
//   if (begin >= g.target.length) {
//     return []
//   } else {
//     let s = sentences(g, begin)
//     return [s].concat(all_sentences(g, s.target.end + 1))
//   }
// }

// /** The subgraph from a subspan

//   let g = init('apa bepa . cepa depa . epa')
//   target_text(subgraph(g, sentences(g, 3))) // => 'cepa depa . '

// */
// pub fn subgraph(g: Graph, s: Subspan): Graph {
//   let source = g.source.slice(s.source.begin, s.source.end + 1)
//   let target = g.target.slice(s.target.begin, s.target.end + 1)
//   let proto_g = {source, target, edges: edge_record([])}
//   let sm = source_map(proto_g)
//   let tm = target_map(proto_g)
//   let edges = record.filter(g.edges, e => e.ids.some(id => sm.has(id) || tm.has(id)))
//   return {source, target, edges}
// }

// pub fn indicies_around_positions(
//   g: Graph,
//   side: Side,
//   positions: number[]
// ): {side: Side; index: number}[] {
//   let N = target_text(g).length
//   let nearby = Utils.flatMap(positions, i => [i - 1, i, i + 1])
//   let in_bounds = nearby.filter(i => Utils.within(0, i, N))
//   return in_bounds.map(i => ({
//     side: side,
//     index: T.token_at(get_side_texts(g, side), i).token,
//   }))
// }

// /** Given many graphs on the same source text, find the overlapping sentence groups

// Uses merge_series which is very inefficient
// */
// pub fn sentence_groups<K extends string>(gs: Record<K, Graph>): Record<K, Subspan>[] {
//   return Utils.merge_series(
//     record.map(gs, g => all_sentences(g)),
//     subspan_merge,
//     R.eqProps('source')
//   )
// }

// /** Get a set of used labels.

//   let g = {
//     source: [{id: 's0', text: 'x '}, {id: 's1', text: 'y '}],
//     target: [{id: 't0', text: 'x '}, {id: 't1', text: 'y '}],
//     edges: {
//       'e-s0-t0': {id: 'e-s0-t0', ids: ['s0', 't0'], labels: ['A', 'B'], manual: false},
//       'e-s1-t1': {id: 'e-s1-t1', ids: ['s1', 't1'], labels: ['A'], manual: false},
//     }
//   }
//   used_labels(g).sort() // => ['A', 'B']

// */
// pub fn used_labels(g: Graph): string[] {
//   return Utils.uniq(Utils.flatMap(Object.values(g.edges), e => e.labels))
// }

// /** Whether a label permits a comment.

//   is_comment_label('pl') // => false
//   is_comment_label('!') // => true
//  */
// pub fn is_comment_label(label: string): boolean {
//   return label.indexOf('!') != -1
// }

// /** Modify the labels at an identifier

//   let g = init('word')
//   let g2 = modify_labels(g, 'e-s0-t0', (labels: string[]) => [...labels, 'ABC'])
//   let g3 = modify_labels(g2, 'e-s0-t0', (labels: string[]) => [...labels, 'DEF'])
//   g3.edges['e-s0-t0'].labels // => ['ABC', 'DEF']

// */
// pub fn modify_labels(g: Graph, edge_id: string, k: (labels: string[]) => string[]): Graph {
//   let store = Store.init(g)
//   let edge = edge_store(store, edge_id)
//   edge.modify(e => {
//     let labels = k(e.labels)
//     let comment = labels.some(is_comment_label) ? e.comment : undefined
//     return Edge(e.ids, labels, e.manual, comment)
//   })
//   return store.get()
// }

// pub fn comment_edge(g: Graph, edge_id: string, comment?: string) {
//   let store = Store.init(g)
//   let edge = edge_store(store, edge_id)
//   edge.modify(e => Edge(e.ids, e.labels, e.manual, comment))
//   return store.get()
// }

// pub fn edge_store(g: Store<Graph>, edge_id: string): Store<Edge> {
//   return g
//     .at('edges')
//     .via(Lens.key(edge_id))
//     .via(Lens.def(Edge([], [])))
// }

// /** Normalize the unique identifiers in this graph. Use before comparing deep equality.

//   let g = modify_tokens(init('apa bepa cepa '), 1, 2, 'depa epa ')
//   normalize(normalize(g)) // => normalize(g)

//   // new graphs are in normal form, except that they are not marked as manual
//   let g = init('apa bepa cepa ')
//   normalize(g, 'keep') // => g

//   let g = init('x')
//   let ab = {
//     source: [{id: 'a0', text: 'x '}],
//     target: [{id: 'b0', text: 'x '}],
//     edges: {'e-a0-b0': {id: 'e-a0-b0', ids: ['a0', 'b0'], labels: [], manual: false}}
//   }
//   normalize(g, 'keep', 'a', 'b') // => ab

//   let g = init('x')
//   let same = {
//     source: [{id: '0', text: 'x '}],
//     target: [{id: '1', text: 'x '}],
//     edges: {'e-0-1': {id: 'e-0-1', ids: ['0', '1'], labels: [], manual: false}}
//   }
//   normalize(g, 'keep', '', '') // => same

// */
// pub fn normalize(
//   g: Graph,
//   set_manual_to: boolean | 'keep' = true,
//   s_prefix = 's',
//   t_prefix = 't'
// ): Graph {
//   let rev = {} as Record<string, string>
//   let rn = Utils.Renumber<string>()
//   g.source.forEach(tk => rn.num(tk.id))
//   g.target.forEach(tk => rn.num(tk.id))
//   let N = g.source.length
//   let new_id = (id: string) => {
//     let i = rn.num(id)
//     return i < N || s_prefix == t_prefix ? s_prefix + i : t_prefix + (i - N)
//   }
//   let source = g.source.map(s => Token(s.text, new_id(s.id)))
//   let target = g.target.map(s => Token(s.text, new_id(s.id)))
//   let edges = R.fromPairs(
//     record.traverse(g.edges, e => {
//       let E = Edge(
//         e.ids.map(new_id),
//         e.labels.sort(),
//         set_manual_to === 'keep' ? e.manual : set_manual_to,
//         e.comment
//       )
//       return [E.id, E] as [string, Edge]
//     })
//   )
//   return {source, target, edges, ...(g.comment ? {comment: g.comment} : {})}
// }

// pub fn equal(g1: Graph, g2: Graph, set_manual_to: boolean | 'keep' = true): boolean {
//   return R.equals(normalize(g1, set_manual_to), normalize(g2, set_manual_to))
// }

// /** Make all trailing whitespace of a specific form */
// pub fn normalize_whitespace(g: Graph, ws = ' '): Graph {
//   let on_tok = (s: Token) => Token((s.text.match(/\S+/) || [''])[0] + ws, s.id)
//   return {...g, source: g.source.map(on_tok), target: g.target.map(on_tok)}
// }

// /** Sets the target text to the source text, but preserving all labels and comments */
// pub fn source_to_target(g: Graph, make_manual: boolean = true): Graph {
//   let i = next_id(g)
//   let rename_map: Record<string, string> = {}
//   let target = g.source.map(s => {
//     let id = 't' + i++
//     rename_map[s.id] = id
//     return Token(s.text, id)
//   })
//   let edges = Utils.flatMap(Object.values(g.edges), e => {
//     let ids = Utils.flatMap(e.ids, sid => {
//       let tid = rename_map[sid]
//       if (tid) {
//         return [sid, tid]
//       } else {
//         return []
//       }
//     })
//     if (ids.length > 0) {
//       return [Edge(ids, e.labels, make_manual, e.comment)]
//     } else {
//       return []
//     }
//   })
//   return {source: g.source, target, edges: edge_record(edges)}
// }

// /* Sort edge labels according to some order */
// pub fn sort_edge_labels(g: Graph, order: (label: string) => number): Graph {
//   return {
//     ...g,
//     edges: record.map(g.edges, e => Edge(e.ids, R.sortBy(order, e.labels), e.manual, e.comment)),
//   }
// }

// /** Map from labels to edges where they are used.

//   let g = modify_labels(init('apa bepa'), 'e-s1-t1', () => ['L', 'LL'])
//   label_edge_map(g, l => l.length > 1) // => {'LL': [g.edges['e-s1-t1']]}
//   label_edge_map(g) // => {'L': [g.edges['e-s1-t1']], 'LL': [g.edges['e-s1-t1']]}
//  */
// pub fn label_edge_map(g: Graph, filter?: (l: string) => boolean): Record<string, Edge[]> {
//   let label_edge_map: Record<string, Edge[]> = {}
//   record.forEach(g.edges, e =>
//     e.labels.forEach(l => (!filter || filter(l)) && Utils.push(label_edge_map, l, e))
//   )
//   return label_edge_map
// }
