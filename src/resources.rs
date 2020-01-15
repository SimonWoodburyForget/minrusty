
pub struct Quit(pub bool);

impl Default for Quit {
    fn default() -> Self {
        Quit(false)
    }
}
