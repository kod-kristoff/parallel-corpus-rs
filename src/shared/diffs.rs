pub fn hdiff<A, B, CA, CB>(xs: &[A], ys: &[B], a_cmp: CA, b_cmp: CB) -> Vec<Change<A, B>>
where
    CA: Fn(&A, &str) -> String,
    CB: Fn(&B, &str) -> String,
{
}

fn char_stream() -> AllChars {
    AllChars::default()
}

#[derive(Debug, Default)]
struct AllChars {
    i: u32,
}

impl Iterator for AllChars {
    type Item = char;
}
