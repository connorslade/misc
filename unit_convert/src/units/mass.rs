use crate::impl_units;

impl_units! {
    Mass => {
        Gram => [
            <| |g| *g,
            |> |g| *g,
            aliases = ["g"],
            metric = true
        ],
        Tonne => [
            <| |t| t * 1000000.0,
            |> |g| g / 100000.0,
            aliases = ["t"]
        ],
        Pound => [
            <| |lb| lb * 453.59237,
            |> |g| g / 453.59237,
            aliases = ["lb"]
        ]
    }
}
