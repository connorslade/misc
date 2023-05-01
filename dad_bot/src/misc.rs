pub fn dadable(msg: &str) -> bool {
    const DAD_TRIGGER: &[&str] = &["im", "i am", "i'm", "i’m", "i’m"];
    const DAD_ANTI_TRIGGER: &[&str] = &["(shut)", "(stfu)", "(no)"];

    let msg = msg.to_lowercase();
    DAD_TRIGGER.iter().any(|&x| msg.contains(x))
        && !DAD_ANTI_TRIGGER.iter().any(|&x| msg.contains(x))
}
