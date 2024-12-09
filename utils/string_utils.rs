pub fn remove_non_alphanumeric(s: &str) -> String {
   s.chars().filter(|c| c.is_alphanumeric()).collect()
}
