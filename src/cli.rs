use dialoguer::Select;

pub fn select_item(prompt: &str, items: &[&str]) -> usize {
    Select::new()
        .with_prompt(prompt)
        .items(items)
        .default(0)
        .interact()
        .unwrap()
}
