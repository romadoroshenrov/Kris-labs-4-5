//! Small, independent teaching module based on the command names in the supplied Kris report.
//! It does not contain or replace the original voice assistant source code.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Command {
    OpenBrowser,
    OpenTaskManager,
    LockWorkstation,
    TakeScreenshot,
}

pub fn resolve(input: &str) -> Option<Command> {
    let normalized = input
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();

    if normalized == "открой браузер" || normalized == "запусти браузер" {
        Some(Command::OpenBrowser)
    } else if normalized == "открой диспетчер задач"
        || normalized == "запусти диспетчер задач"
    {
        Some(Command::OpenTaskManager)
    } else if normalized == "заблокируй компьютер"
        || normalized == "заблокируй станцию"
    {
        Some(Command::LockWorkstation)
    } else if normalized == "сделай скриншот"
        || normalized == "создай скриншот"
    {
        Some(Command::TakeScreenshot)
    } else {
        None
    }
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
