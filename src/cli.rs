use dialoguer::Select;

pub fn select_item<'a, T: ToString>(prompt: &str, items: &'a [T]) -> &'a T {
    let selected_index = Select::new()
        .with_prompt(prompt)
        .items(items)
        .default(0)
        .interact()
        .unwrap();

    &items[selected_index]
}
