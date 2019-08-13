use crate::models::env_prompt::EnvironmentPrompt;
use console::Style;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Confirmation, Input, Select};
use std::env;

/// The EnvironmentPromptHandler will prompt the user for values to use for ENVIRONMENT variables.
#[derive(Debug, Default)]
pub struct EnvironmentPromptHandler {}

impl EnvironmentPromptHandler {
    // Process a EnvironmentPrompt list.
    pub fn process(env_model: &Vec<EnvironmentPrompt>) {
        for prompt in env_model {
            let context = match &prompt.context {
                Some(context) => context.clone(),
                None => format!("{} should be set?", &prompt.name),
            };
            match &prompt.kind {
                Some(x) => {
                    if x == "input" {
                        env::set_var(
                            &prompt.name,
                            EnvironmentPromptHandler::ask_input(
                                &context,
                                &prompt.default,
                                &prompt.name,
                                &prompt.options,
                            ),
                        );
                    } else if x == "option" {
                        env::set_var(
                            &prompt.name,
                            EnvironmentPromptHandler::ask_with_options(
                                &context,
                                &prompt.default,
                                &prompt.options,
                            ),
                        );
                    } else {
                        env::set_var(
                            &prompt.name,
                            EnvironmentPromptHandler::ask_bool(&context).to_string(),
                        );
                    }
                }
                None => {
                    env::set_var(
                        &prompt.name,
                        EnvironmentPromptHandler::ask_bool(&context).to_string(),
                    );
                }
            }
        }
    }

    /// Check if a ENVIRONMENT value exist.
    fn check_for_existing_value(value_name: &String) -> String {
        match env::var(value_name) {
            Ok(val) => val,
            Err(_e) => {
                // println!("{}", _e);
                String::new()
            }
        }
    }

    // Prompt with list of options.
    pub fn ask_with_options(
        context: &String,
        default_value: &Option<String>,
        value_options: &Option<Vec<String>>,
    ) -> String {
        let color_theme = ColorfulTheme::default();
        let default = match default_value {
            Some(x) => x.clone(),
            None => "".to_string(),
        };
        match value_options {
            Some(x) => {
                let response = Select::with_theme(&color_theme)
                    .with_prompt(&context)
                    .default(x.iter().position(|x| x == &default).unwrap_or(0))
                    .items(&x[..])
                    .interact()
                    .unwrap();

                x[response].to_owned()
            }
            None => {
                eprintln!("Cannot ask question without options.");
                String::new()
            }
        }
    }

    // Prompt with boolean question.
    fn ask_bool(context: &String) -> bool {
        let color_theme = ColorfulTheme {
            yes_style: Style::new().cyan(),
            no_style: Style::new().cyan(),
            ..ColorfulTheme::default()
        };
        Confirmation::with_theme(&color_theme)
            .with_text(&context)
            .interact()
            .unwrap()
    }

    // Prompt for user defined input.
    fn ask_input(
        context: &String,
        default_value: &Option<String>,
        env_name: &String,
        value_options: &Option<Vec<String>>,
    ) -> String {
        let mut question = if context.len() > 0 {
            context.to_string()
        } else {
            format!("Set value for {}", env_name)
        };
        let color_theme = ColorfulTheme::default();
        let mut prompt = Input::with_theme(&color_theme);

        match default_value {
            Some(x) => {
                let default_env = EnvironmentPromptHandler::check_for_existing_value(&env_name);
                if default_env.len() > 0 {
                    prompt.default(x.as_str().to_owned());
                } else {
                    prompt.default(x.as_str().to_owned());
                }
            }
            None => { /* Do nothing. */ }
        }
        match value_options {
            Some(x) => {
                let options = format!(".\nOptions: {:?}", x);
                question.push_str(options.as_str());
            }
            None => {
                question.push_str(":");
            }
        }

        prompt
            .with_prompt(&question.as_str())
            .interact()
            .unwrap()
            .to_owned()
    }
}
