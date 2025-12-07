pub fn cartesian_product<X: Clone, Y>(
    xs: impl Iterator<Item = X>,
    ys: impl Iterator<Item = Y> + Clone,
) -> impl Iterator<Item = (X, Y)> {
    xs.flat_map(move |x| ys.clone().map(move |y| (x.clone(), y)))
}
