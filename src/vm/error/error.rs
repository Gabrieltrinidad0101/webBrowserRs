use crate::share::code::CODE;

#[derive(Debug)]
pub enum ErrorKind {
    Dot,
    UnexpectedChar,
    IllegalCharacter
}

#[derive(Debug)]
pub struct CustomError {
    pub kind: ErrorKind,
    pub description: String,
    pub body: String,
}

pub fn error(kind: ErrorKind, ln: usize, col: usize, idx: usize, len: usize) -> CustomError {
    let message = match kind {
        ErrorKind::Dot            => "A number cannot have two dots",
        ErrorKind::UnexpectedChar => "Unexpected character",
        ErrorKind::IllegalCharacter => "Illegal character"
    };
    let error = format!("\n{}{}", " ".repeat(col -1), "^".repeat(len));
    let code = CODE.read().unwrap();
    let mut show_error: String = (code[idx] as char).to_string();
    let mut index_back: usize = idx - 1;
    let mut index_front: usize = idx + 1;
    let mut lines_count = 0;
    while index_back > 0 {
        if lines_count == 3 {
            break;
        }
        if code[index_back] == b'\n' || code[index_back] == b'\r' {
            show_error.insert_str(0, &("{}: ".to_string() + &(ln - lines_count).to_string()));
            lines_count += 1;
        }
        show_error.insert(0, code[index_back] as char);
        index_back -= 1;
    }

    lines_count = 0;

    while index_front < code.len() {
        if lines_count == 3 {
            break;
        }
        show_error.push(code[index_front] as char);
        if code[index_front] == b'\n' || code[index_front] == b'\r' {
            if lines_count == 0 {
                show_error += &error;
            }
            lines_count += 1;
            show_error.push_str(&("{}: ".to_string() + &(ln + lines_count).to_string()))
        }
        index_front += 1;
    }

    if lines_count == 0 {
        show_error += &error;
    }

    CustomError {
        kind,
        description: message.to_string(),
        body: show_error,
    }
}
