pub fn example() {
    assert_eq!(
        generate_nametag_text("Beyoncé".into()),
        Ok("Hi! My name is Beyoncé".into())
    );
    assert_eq!(
        generate_nametag_text("".into()),
        Err("`name` was empty; it must be nonempty.".into())
    );
}

fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.len() > 0 {
        Ok(format!("Hi! My name is {}", name))
    } else {
        // Empty names aren't allowed.
        Err(String::from("`name` was empty; it must be nonempty."))
    }
}
