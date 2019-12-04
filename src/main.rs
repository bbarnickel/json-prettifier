use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    let mut prettifier = Prettifier::new();
    let stdout = io::stdout();
    let mut locked_stdout = stdout.lock();

    while let Ok(n) = stdin.read_line(&mut buffer) {
        if n == 0 {
            break;
        }

        let result = prettifier.process(&buffer);
        
        locked_stdout.write(result.as_bytes()).expect("Cannot write!");
    }
}

struct Prettifier {
    indent: usize,
    indent_string: &'static str,
    within_literal: bool
}

impl Prettifier {
    fn new() -> Self {
        Prettifier {
            indent: 0,
            indent_string: "    ",
            within_literal: false
        }
    }

    fn push_newline_and_delim(&self, string: &mut String) {
        string.push('\n');
        for _ in 0..self.indent {
            string.push_str(self.indent_string);
        }
    }

    fn process(&mut self, line: &str) -> String {
        let mut result = String::with_capacity(line.len());

        for c in line.chars() {
            if c == '"' {
                self.within_literal = !self.within_literal;
            }

            if self.within_literal
            {
                result.push(c);
                continue;
            }

            if c == '}' || c == ']' {
                self.indent -= 1;
                self.push_newline_and_delim(&mut result);
            }

            result.push(c);

            if c == '{' || c == '[' {
                self.indent += 1;
                self.push_newline_and_delim(&mut result);
            }

            if c == ':' {
                result.push(' ');
            }

            if c == ',' {
                self.push_newline_and_delim(&mut result);
            }
        }

        result
    }
}
