use crate::impl_units;

impl_units! {
    Time => {
        Second => [
            <| |s| *s,
            |> |s| *s,
            aliases = ["s", "sec"],
            metric = true
        ],
        Minute => [
            <| |m| m * 60.0,
            |> |s| s / 60.0,
            aliases = ["min"]
        ],
        Hour => [
            <| |h| h * 3600.0,
            |> |s| s / 3600.0,
            aliases = ["h", "hr"]
        ],
        Week => [
            <| |w| w * 604800.0,
            |> |s| s / 604800.0,
            aliases = ["wk"]
        ],
        /// Apparent interval between two successive returns of the Sun to the same meridian
        Sol => [
            <| |s| s * 88740.244,
            |> |s| s / 88740.244
        ]
    }
}
