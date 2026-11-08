//! [confirmed:W027,archaeology011a341] History navigation; no invented actions.
//! [designed:T73] 77 columns,18 lines and clamped line/page scrolling.
use crate::timeline::Event;
pub const PAGE_LINES: usize = 18;
pub const COLUMNS: usize = 77;
pub fn lines(events: &[Event]) -> Vec<String> {
    let mut output = Vec::new();
    for event in events {
        let text = format!("{} {}", event.native["minute"], event.text);
        for paragraph in text.split('\n') {
            let mut line = String::new();
            for word in paragraph.split_whitespace() {
                let chars: Vec<_> = word.chars().collect();
                for chunk in chars.chunks(COLUMNS) {
                    if !line.is_empty() && line.chars().count() + 1 + chunk.len() > COLUMNS {
                        output.push(std::mem::take(&mut line));
                    }
                    if !line.is_empty() {
                        line.push(' ');
                    }
                    line.extend(chunk);
                }
            }
            if !line.is_empty() {
                output.push(line);
            }
        }
    }
    output
}
