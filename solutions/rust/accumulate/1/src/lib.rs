pub fn map<T, A>(input: Vec<T>, mut function: impl FnMut(T) -> A) -> Vec<A> {
    input.into_iter().map(|v| function(v)).collect()
}
