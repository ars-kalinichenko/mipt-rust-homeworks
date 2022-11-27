use anyhow::{bail, Context, Result};
use std::{fs, path::Path};

#[allow(clippy::enum_variant_names)]
enum TokenKind {
    Private,
    BeginPrivate,
    EndPrivate,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TokenProperty {
    NoHint,
    Unimplemented,
}

struct Token {
    kind: TokenKind,
    properties: Vec<TokenProperty>,
}

fn parse_token(line: &str) -> Result<Option<Token>> {
    let comment = match line.find("//") {
        Some(pos) => &line[pos..],
        None => return Ok(None),
    };

    let cmd = match comment.find("compose::") {
        Some(pos) => &comment[pos + "compose::".len()..],
        None => return Ok(None),
    };

    let kind = if cmd.starts_with("private") {
        TokenKind::Private
    } else if cmd.starts_with("begin_private") {
        TokenKind::BeginPrivate
    } else if cmd.starts_with("end_private") {
        TokenKind::EndPrivate
    } else {
        bail!("unknown compose command: {}", cmd);
    };

    let properties_str = match cmd.find('(') {
        Some(pos) => {
            if !cmd.trim_end().ends_with(')') {
                bail!("unclosed '('");
            }
            &cmd[pos + 1..cmd.trim_end().len() - 1]
        }
        None => "",
    };

    let mut properties = vec![];
    for prop in properties_str.split_inclusive(',') {
        match prop {
            "no_hint" => properties.push(TokenProperty::NoHint),
            "unimplemented" => properties.push(TokenProperty::Unimplemented),
            s => bail!("unknown property: {}", s),
        }
    }

    Ok(Some(Token { kind, properties }))
}

fn find_token(lines: &[&str], start: usize) -> Result<Option<(usize, Token)>> {
    for (i, line) in lines[start..].iter().enumerate() {
        let mb_token = parse_token(line)
            .with_context(|| format!("failed to parse token on line {}", i + 1))?;
        if let Some(token) = mb_token {
            return Ok(Some((i + start, token)));
        }
    }
    Ok(None)
}

fn process_source(src: String) -> Result<String> {
    let mut dst = String::new();

    let lines = src.lines().collect::<Vec<_>>();
    let mut next_pos = 0;
    while let Some((begin, token)) = find_token(&lines, next_pos)? {
        let end = match token.kind {
            TokenKind::EndPrivate => bail!("unpaired 'end_private' on line {}", begin + 1),
            TokenKind::Private => begin + 1,
            TokenKind::BeginPrivate => {
                let mut pos = begin + 1;
                let mut mb_end: Option<usize> = None;
                while let Some((k, token)) = find_token(&lines, pos)? {
                    match token.kind {
                        TokenKind::BeginPrivate => {
                            bail!("nested 'begin_private' on line {}", k + 1)
                        }
                        TokenKind::Private => pos = k + 1,
                        TokenKind::EndPrivate => {
                            mb_end = Some(k);
                            break;
                        }
                    }
                }
                match mb_end {
                    Some(end) => end + 1,
                    None => bail!("unclosed 'begin_private' on line {}", begin + 1),
                }
            }
        };

        #[allow(clippy::needless_range_loop)]
        for i in next_pos..begin {
            dst += lines[i];
            dst += "\n";
        }

        let no_hint = token.properties.contains(&TokenProperty::NoHint);
        let unimpl = token.properties.contains(&TokenProperty::Unimplemented);
        if no_hint {
            if begin > 0
                && lines[begin - 1].trim().is_empty()
                && end < lines.len()
                && lines[end].trim().is_empty()
            {
                next_pos = end + 1;
            } else {
                next_pos = end;
            }
        } else {
            let mut insert_line = |line: &str| {
                for c in lines[begin].chars() {
                    if c.is_whitespace() {
                        dst.push(c);
                    } else {
                        break;
                    }
                }
                dst.push_str(line);
            };

            insert_line("// TODO: your code goes here.\n");
            if unimpl {
                insert_line("unimplemented!()\n");
            }

            next_pos = end;
        }
    }

    for line in &lines[next_pos..] {
        dst += line;
        dst += "\n";
    }

    Ok(dst)
}

pub fn process_file(input: &Path, output: &Path) -> Result<()> {
    let out_dir = output.parent().unwrap();
    fs::create_dir_all(out_dir).context("failed to create directory")?;
    if input.to_str().map(|s| s.ends_with(".rs")).unwrap_or(false) {
        let content = fs::read_to_string(input)
            .with_context(|| format!("failed to read file {:?}", input))?;
        let new_content = process_source(content)
            .with_context(|| format!("failed to process file {:?}", input))?;
        fs::write(output, new_content).with_context(|| format!("failed to write file {:?}", input))
    } else {
        fs::copy(input, output)
            .map(|_| ())
            .with_context(|| format!("failed to copy {:?} to {:?}", input, output))
    }
}
