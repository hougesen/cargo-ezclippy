use convert_case::{Case, Casing};

const WHITESPACE: &str = "    ";

#[derive(serde::Deserialize)]
struct Lint {
    id: String,
    group: String,
}

fn get_lint_groups() -> std::collections::HashMap<String, std::collections::HashSet<String>> {
    let lints = reqwest::blocking::get("https://rust-lang.github.io/rust-clippy/master/lints.json")
        .unwrap()
        .json::<Vec<Lint>>()
        .unwrap();

    let mut rules = std::collections::HashMap::<String, std::collections::HashSet<String>>::new();

    for lint in lints {
        rules.entry(lint.group).or_default().insert(lint.id);
    }

    rules
}

fn generate_lint_struct(name: &str, group: &std::collections::HashSet<String>) -> String {
    let serde_default = format!("{WHITESPACE}#[serde(default)]\n");
    let serde_skip = format!("{WHITESPACE}#[serde(skip_serializing_if = \"Option::is_none\")]\n");
    let mut rules = group
        .iter()
        .map(|rule| {
            format!(
                "{serde_default}{serde_skip}{WHITESPACE}pub {rule}: Option<crate::lints::LintLevel>,"
            )
        })
        .collect::<Vec<_>>();

    rules.sort_unstable();

    format!(
        "#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Default)]
#[serde(default)]
pub struct {name} {{
{}
}}
",
        rules.join("\n")
    )
}

fn generate_rules_struct(
    lints: std::collections::HashMap<String, std::collections::HashSet<String>>,
) {
    let mut lint_structs: Vec<String> = Vec::new();

    let mut lint_groups: Vec<String> = Vec::new();

    for (name, rules) in lints {
        let struct_name = format!("{name}_group").to_case(Case::Pascal);

        lint_structs.push(generate_lint_struct(&struct_name, &rules));

        lint_groups.push(format!("{WHITESPACE}pub {name}: {struct_name},"));
    }

    lint_structs.sort_unstable();

    lint_groups.sort_unstable();

    let file = format!(
        "{}
#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Default)]
pub struct Config {{
{WHITESPACE}#[serde(rename = \"$schema\", default = \"crate::config::default_schema_location\")]
{WHITESPACE}pub schema: String,

{}
}}
",
        lint_structs.join("\n"),
        lint_groups.join("\n")
    );

    std::fs::write("src/generated.rs", file).unwrap();
}

fn main() {
    let lints = get_lint_groups();

    generate_rules_struct(lints);
}
