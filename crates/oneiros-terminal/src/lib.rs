use std::io::IsTerminal;

const NAME_PROMPT: &str = "What should we call you?";

pub struct TerminalOps;

impl TerminalOps {
    pub fn get_name(&self) -> Option<String> {
        if self.is_interactive() {
            let default = detect_name();

            let mut prompt = inquire::Text::new(NAME_PROMPT);

            if let Some(ref default) = default {
                prompt = prompt.with_default(default);
            }

            prompt.prompt().ok()
        } else {
            None
        }
    }

    fn is_interactive(&self) -> bool {
        std::io::stdin().is_terminal() && std::io::stdout().is_terminal()
    }
}

fn detect_name() -> Option<String> {
    std::process::Command::new("git")
        .args(["config", "user.name"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|output| {
            String::from_utf8(output.stdout)
                .ok()
                .map(|given_name| given_name.trim().to_string())
                .filter(|given_name| !given_name.is_empty())
        })
        .or_else(|| Some(whoami::realname()))
}
