use crate::cli::Completions;

pub struct CompletionProcess {}

impl CompletionProcess {
    pub fn run(cli_settings: Completions) -> bool {
        let it_ran = match cli_settings {
            Completions::Bash(_x) => {
                super::Completions::bash();
                true
            }
            Completions::Fish(_x) => {
                super::Completions::fish();
                true
            }
            Completions::Zsh(_x) => {
                super::Completions::zsh();
                true
            }
            Completions::PowerShell(_x) => {
                super::Completions::powershell();
                true
            }
            Completions::Elvish(_x) => {
                super::Completions::elvish();
                true
            }
        };
        it_ran
    }
}
