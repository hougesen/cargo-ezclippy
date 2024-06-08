use crate::lints::Lint;

pub fn build_config(
    lints: Vec<Lint>,
) -> std::collections::HashMap<String, std::collections::HashMap<String, String>> {
    let mut rules =
        std::collections::HashMap::<String, std::collections::HashMap<String, String>>::new();

    for lint in lints {
        rules
            .entry(lint.group)
            .or_default()
            .insert(lint.id, lint.level);
    }

    rules
}
