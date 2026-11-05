//! Small, independent teaching module based on the command names in the supplied Kris report.
//! It does not contain or replace the original voice assistant source code.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Command {
    OpenBrowser,
    OpenTaskManager,
    LockWorkstation,
    TakeScreenshot,
}

// Related aliases and their result are now kept together as data.
struct CommandSpec {
    command: Command,
    aliases: &'static [&'static str],
}

const COMMANDS: &[CommandSpec] = &[
    CommandSpec {
        command: Command::OpenBrowser,
        aliases: &["открой браузер", "запусти браузер"],
    },
    CommandSpec {
        command: Command::OpenTaskManager,
        aliases: &["открой диспетчер задач", "запусти диспетчер задач"],
    },
    CommandSpec {
        command: Command::LockWorkstation,
        aliases: &["заблокируй компьютер", "заблокируй станцию"],
    },
    CommandSpec {
        command: Command::TakeScreenshot,
        aliases: &["сделай скриншот", "создай скриншот"],
    },
];

fn normalize(input: &str) -> String {
    input
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

struct CommandResolver;

impl CommandResolver {
    fn resolve(&self, input: &str) -> Option<Command> {
        let normalized = normalize(input);
        COMMANDS
            .iter()
            .find(|spec| spec.aliases.contains(&normalized.as_str()))
            .map(|spec| spec.command)
    }
}

pub fn resolve(input: &str) -> Option<Command> {
    CommandResolver.resolve(input)
}

#[cfg(test)]
mod tests {
    use super::{resolve, Command};

    #[test]
    fn known_commands_and_aliases() {
        let cases = [
            ("открой браузер", Command::OpenBrowser),
            ("запусти браузер", Command::OpenBrowser),
            ("открой диспетчер задач", Command::OpenTaskManager),
            ("запусти диспетчер задач", Command::OpenTaskManager),
            ("заблокируй компьютер", Command::LockWorkstation),
            ("заблокируй станцию", Command::LockWorkstation),
            ("сделай скриншот", Command::TakeScreenshot),
            ("создай скриншот", Command::TakeScreenshot),
        ];
        for (phrase, expected) in cases {
            assert_eq!(resolve(phrase), Some(expected));
        }
    }

    #[test]
    fn normalization_and_unknown_command() {
        assert_eq!(resolve("  ОТКРОЙ   БРАУЗЕР  "), Some(Command::OpenBrowser));
        assert_eq!(resolve("покажи погоду"), None);
        assert_eq!(resolve(""), None);
    }
}
