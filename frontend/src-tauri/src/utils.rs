pub fn format_timestamp(seconds: f64) -> String {
    let total_seconds = seconds as u64;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let secs = total_seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, secs)
}

pub(crate) const MAX_LOG_RECORD_CHARS: usize = 4_096;

struct CappedLogLine {
    value: String,
    chars: usize,
    max_chars: usize,
    truncated: bool,
}
impl CappedLogLine {
    fn new(max_chars: usize) -> Self {
        Self {
            value: String::new(),
            chars: 0,
            max_chars,
            truncated: false,
        }
    }

    fn finish(mut self) -> String {
        if self.truncated {
            self.value.push('…');
        }
        self.value
    }
}

impl std::fmt::Write for CappedLogLine {
    fn write_str(&mut self, value: &str) -> std::fmt::Result {
        for character in value.chars() {
            if self.chars == self.max_chars {
                self.truncated = true;
                break;
            }
            self.value
                .push(if character.is_control() { ' ' } else { character });
            self.chars += 1;
        }
        Ok(())
    }
}

pub(crate) fn bounded_log_line(arguments: std::fmt::Arguments<'_>) -> String {
    let mut line = CappedLogLine::new(MAX_LOG_RECORD_CHARS);
    let _ = std::fmt::write(&mut line, arguments);
    line.finish()
}

pub(crate) fn log_snippet(value: &str, max_chars: usize) -> String {
    use std::fmt::Write;

    let mut snippet = CappedLogLine::new(max_chars);
    let _ = snippet.write_str(value);
    snippet.finish()
}

#[cfg(test)]
mod tests {
    use super::{bounded_log_line, log_snippet, MAX_LOG_RECORD_CHARS};

    #[test]
    fn logging_helpers_bound_content() {
        assert_eq!(log_snippet("short", 10), "short");
        assert_eq!(log_snippet("résumé", 3), "rés…");
        assert_eq!(log_snippet("a\r\nb\u{7f}", 5), "a  b ");
        assert_eq!(log_snippet("nonempty", 0), "…");
        assert_eq!(bounded_log_line(format_args!("a\r\nb")), "a  b");

        let line = bounded_log_line(format_args!(
            "response_bytes={}, meeting_name_configured={}",
            "REMOTE_BODY_SECRET".len(),
            Some("MEETING_TITLE_SECRET").is_some()
        ));
        for marker in ["REMOTE_BODY_SECRET", "MEETING_TITLE_SECRET"] {
            assert!(!line.contains(marker));
        }

        let oversized = "x".repeat(10_000_001);
        let line = bounded_log_line(format_args!("{}\r\n", oversized));
        assert_eq!(line.chars().count(), MAX_LOG_RECORD_CHARS + 1);
        assert!(line.ends_with('…'));
    }
}

/// Opens macOS System Settings to a specific privacy preference pane
#[cfg(target_os = "macos")]
#[tauri::command]
pub async fn open_system_settings(preference_pane: String) -> Result<(), String> {
    use std::process::Command;

    // Construct the URL for System Settings
    let url = format!("x-apple.systempreferences:com.apple.preference.security?{}", preference_pane);

    // Use the 'open' command on macOS to open the URL
    Command::new("open")
        .arg(&url)
        .spawn()
        .map_err(|e| format!("Failed to open system settings: {}", e))?;

    Ok(())
} 