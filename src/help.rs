use crate::Flag;

/// Trims, concatenates, performs line wrapping, and indents doc comments.
fn add_doccoments(buf: &mut String, docs: &[&str], indent_level: usize) {
    let mut chars = 0;
    for d in docs {
        let d = d.trim();

        if d.is_empty() {
            buf.push('\n');

            chars = 0;
            continue;
        }

        let mut iter = d.split_ascii_whitespace().peekable();

        while let Some(w) = iter.peek() {
            if chars == 0 {
                for _ in 0..indent_level {
                    buf.push(' ');
                }

                *buf += w;
                chars = w.len() + indent_level;
                iter.next();
            } else if chars + w.len() < 80 {
                buf.push(' ');
                *buf += w;
                chars += 1 + w.len();
                iter.next();
            } else {
                buf.push('\n');
                chars = 0;
            }
        }
    }

    if !docs.is_empty() {
        *buf += "\n\n";
    }
}

/// More complicated runtime formatting of comandline options.
///
/// This will automatically trim whitespace, indent, and perform line wrapping.
pub fn wrapping_format(buf: &mut String, docs: &[Flag]) {
    for flag in docs {
        if flag.flags.is_empty() {
            continue;
        }

        *buf += " ";

        for flag in flag.flags {
            *buf += " ";
            *buf += flag.trim();
        }

        for param in flag.params {
            *buf += " <";
            *buf += param;
            *buf += ">";
        }

        *buf += "\n";

        add_doccoments(buf, flag.doc, 4);
    }
}
