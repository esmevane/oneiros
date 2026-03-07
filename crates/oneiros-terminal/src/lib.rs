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

    /// Prompt for TOFU acceptance of a new peer's certificate fingerprint.
    /// Returns true if accepted, false otherwise. Non-interactive sessions always refuse.
    pub fn prompt_tofu(&self, endpoint: &str, fingerprint: &str) -> bool {
        if !self.is_interactive() {
            return false;
        }
        let message = format!(
            "First connection to {endpoint}\n  Certificate fingerprint: {fingerprint}\n  Accept this peer?"
        );
        inquire::Confirm::new(&message)
            .with_default(false)
            .prompt()
            .unwrap_or(false)
    }

    /// Prompt for explicit approval of an insecure (non-TLS) connection.
    /// Returns true if approved, false otherwise. Non-interactive sessions always refuse.
    pub fn prompt_insecure(&self, endpoint: &str) -> bool {
        if !self.is_interactive() {
            return false;
        }
        let message = format!(
            "{endpoint} does not support TLS.\n  \
             Connecting without encryption means traffic is visible on the network.\n  \
             Allow insecure connection?"
        );
        inquire::Confirm::new(&message)
            .with_default(false)
            .prompt()
            .unwrap_or(false)
    }

    /// Display a warning about a changed peer certificate fingerprint.
    pub fn warn_fingerprint_changed(&self, endpoint: &str, expected: &str, actual: &str) {
        eprintln!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@");
        eprintln!("@    WARNING: REMOTE HOST IDENTIFICATION HAS CHANGED!    @");
        eprintln!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@");
        eprintln!("Endpoint: {endpoint}");
        eprintln!("  Stored fingerprint:  {expected}");
        eprintln!("  Current fingerprint: {actual}");
        eprintln!("This could indicate a man-in-the-middle attack.");
        eprintln!("The stored fingerprint must be manually updated to proceed.");
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
