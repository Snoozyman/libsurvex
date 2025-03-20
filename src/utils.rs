pub trait TrimAndLower {
    fn replace_whitespace(&self, to: char) -> String;
    fn lower(&self) -> String;
}
impl TrimAndLower for String {
    fn lower(&self) -> String {
        self.to_string().to_lowercase()
    }
    fn replace_whitespace(&self, to: char) -> String {
        self.to_string().replace(' ', to.to_string().as_str())
    }
}