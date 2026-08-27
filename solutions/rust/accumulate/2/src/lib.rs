pub fn map<T, A>(input: Vec<T>, function: impl FnMut(T) -> A) -> Vec<A> {
    input.into_iter().map(function).collect()
}
