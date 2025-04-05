use crate::plugin::Plugin;

pub struct RoastMe;

impl Plugin for RoastMe {
    fn name(&self) -> &'static str {
        "roast-me"
    }

    fn description(&self) -> &'static str {
        "Gives a custom roast."
    }

    fn execute(&self, _input: String) -> String {
        "You debug with println, don’t you?".to_string()
    }
}
