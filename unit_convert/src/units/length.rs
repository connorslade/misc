use crate::impl_units;

impl_units! {
    Length => {
        Meter => [
            <| |m| *m,
            |> |m| *m,
            aliases = ["m", "metre"],
            metric = true
        ],
        Inch => [
            <| |i| i * 0.0254,
            |> |m| m / 0.0254,
            aliases = ["in"]
        ],
        Thou => [
            <| |th| th * 0.0000254,
            |> |m| m / 0.0000254,
            aliases = ["mil"]
        ],
        Foot => [
            <| |f| f * 0.3048,
            |> |m| m / 0.3048,
            aliases = ["ft", "foot"]
        ],
        Yard => [
            <| |y| y * 0.9144,
            |> |m| m / 0.9144,
            aliases = ["yd"]
        ],
        Mile => [
            <| |mi| mi * 1609.344,
            |> |m| m / 1609.344,
            aliases = ["mi"]
        ],
        League => [
            <| |l| l * 4828.0417,
            |> |m| m / 4828.0417
        ]
    }
}
