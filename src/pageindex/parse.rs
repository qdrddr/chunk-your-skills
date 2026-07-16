#[derive(Debug, Clone)]
pub struct HeaderNode {
    pub title: String,
    pub line_num: usize,
}

#[derive(Debug, Clone)]
pub struct ContentNode {
    pub title: String,
    pub line_num: usize,
    pub level: usize,
    pub text: String,
}

/// Parse a markdown heading line (`#`–`######` + title).
#[must_use]
fn parse_header(stripped: &str) -> Option<(usize, &str)> {
    let bytes = stripped.as_bytes();
    if bytes.is_empty() || bytes[0] != b'#' {
        return None;
    }
    let mut level = 0usize;
    while level < bytes.len() && bytes[level] == b'#' {
        level += 1;
    }
    if level == 0 || level > 6 {
        return None;
    }
    if level >= bytes.len() || bytes[level] != b' ' {
        return None;
    }
    let title = stripped[level..].trim();
    if title.is_empty() {
        return None;
    }
    Some((level, title))
}

#[must_use]
pub fn extract_nodes_from_markdown(markdown_content: &str) -> (Vec<HeaderNode>, Vec<String>) {
    let lines: Vec<String> = markdown_content.lines().map(str::to_string).collect();
    let mut node_list = Vec::new();
    let mut in_code_block = false;

    for (idx, line) in lines.iter().enumerate() {
        let line_num = idx + 1;
        let stripped = line.trim();

        if stripped.starts_with("```") {
            in_code_block = !in_code_block;
            continue;
        }

        if stripped.is_empty() || in_code_block {
            continue;
        }

        if let Some((_, title)) = parse_header(stripped) {
            node_list.push(HeaderNode {
                title: title.to_string(),
                line_num,
            });
        }
    }

    (node_list, lines)
}

#[must_use]
pub fn extract_node_text_content(
    node_list: &[HeaderNode],
    markdown_lines: &[String],
) -> Vec<ContentNode> {
    let mut all_nodes = Vec::new();

    for node in node_list {
        let line_idx = node.line_num.saturating_sub(1);
        let Some(line_content) = markdown_lines.get(line_idx) else {
            continue;
        };
        let Some((level, _)) = parse_header(line_content.trim()) else {
            continue;
        };
        all_nodes.push(ContentNode {
            title: node.title.clone(),
            line_num: node.line_num,
            level,
            text: String::new(),
        });
    }

    let line_nums: Vec<usize> = all_nodes.iter().map(|n| n.line_num).collect();
    for (i, node) in all_nodes.iter_mut().enumerate() {
        let start_line = node.line_num.saturating_sub(1);
        let end_line = if i + 1 < line_nums.len() {
            line_nums[i + 1].saturating_sub(1)
        } else {
            markdown_lines.len()
        };
        node.text = markdown_lines
            .get(start_line..end_line)
            .map(|slice| slice.join("\n").trim().to_string())
            .unwrap_or_default();
    }

    all_nodes
}

use serde_json::{Map, Value};

/// YAML frontmatter and optional body text before the first heading.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SkillPrefix {
    pub frontmatter: Option<String>,
    pub frontmatter_line_num: Option<u32>,
    pub preamble: Option<String>,
    pub preamble_line_num: Option<u32>,
}

fn line_num_from_index(index: usize) -> Option<u32> {
    u32::try_from(index + 1).ok()
}

fn extract_preamble_from_lines(lines: &[&str], start_idx: usize) -> (Option<String>, Option<u32>) {
    let mut preamble_lines = Vec::new();
    let mut first_content_line = None;
    let mut in_code_block = false;

    for (offset, line) in lines[start_idx..].iter().enumerate() {
        let line_num = line_num_from_index(start_idx + offset);
        let stripped = line.trim();
        if stripped.starts_with("```") {
            in_code_block = !in_code_block;
            preamble_lines.push(*line);
            continue;
        }
        if !in_code_block && parse_header(stripped).is_some() {
            break;
        }
        if first_content_line.is_none() && !stripped.is_empty() {
            first_content_line = line_num;
        }
        preamble_lines.push(*line);
    }

    let text = preamble_lines.join("\n").trim().to_string();
    if text.is_empty() {
        (None, None)
    } else {
        (Some(text), first_content_line)
    }
}

/// Extract skill YAML frontmatter and preamble (content before the first heading).
#[must_use]
pub fn extract_skill_prefix(markdown_content: &str) -> SkillPrefix {
    let lines: Vec<&str> = markdown_content.lines().collect();
    let mut start_idx = 0usize;
    while start_idx < lines.len() && lines[start_idx].trim().is_empty() {
        start_idx += 1;
    }

    if start_idx >= lines.len() || lines[start_idx].trim() != "---" {
        let (preamble, preamble_line_num) = extract_preamble_from_lines(&lines, 0);
        return SkillPrefix {
            frontmatter: None,
            frontmatter_line_num: None,
            preamble,
            preamble_line_num,
        };
    }

    let frontmatter_line_num = line_num_from_index(start_idx);
    let mut end_idx = start_idx + 1;
    while end_idx < lines.len() && lines[end_idx].trim() != "---" {
        end_idx += 1;
    }
    if end_idx >= lines.len() {
        return SkillPrefix::default();
    }

    let yaml = lines[(start_idx + 1)..end_idx].join("\n");
    let frontmatter = format!("---\n{yaml}\n---");
    let (preamble, preamble_line_num) = extract_preamble_from_lines(&lines, end_idx + 1);

    SkillPrefix {
        frontmatter: Some(frontmatter),
        frontmatter_line_num,
        preamble,
        preamble_line_num,
    }
}

/// Return the inner YAML body from a fenced frontmatter block (`---` … `---`).
#[must_use]
pub fn frontmatter_yaml_body(frontmatter: &str) -> Option<String> {
    let trimmed = frontmatter.trim();
    if !trimmed.starts_with("---") {
        return None;
    }
    let rest = trimmed.strip_prefix("---")?.trim_start_matches('\n');
    let end = rest.find("\n---")?;
    Some(rest[..end].to_string())
}

fn yaml_value_to_json(value: serde_yaml::Value) -> Value {
    match value {
        serde_yaml::Value::Null => Value::Null,
        serde_yaml::Value::Bool(b) => Value::Bool(b),
        serde_yaml::Value::Number(n) => n.as_i64().map_or_else(
            || {
                n.as_f64().map_or_else(
                    || Value::String(n.to_string()),
                    |f| serde_json::Number::from_f64(f).map_or(Value::Null, Value::Number),
                )
            },
            |i| Value::Number(i.into()),
        ),
        serde_yaml::Value::String(s) => Value::String(s),
        serde_yaml::Value::Sequence(seq) => {
            Value::Array(seq.into_iter().map(yaml_value_to_json).collect())
        }
        serde_yaml::Value::Mapping(map) => {
            let mut obj = Map::new();
            for (key, value) in map {
                if let Some(key) = yaml_key_to_string(key) {
                    obj.insert(key, yaml_value_to_json(value));
                }
            }
            Value::Object(obj)
        }
        serde_yaml::Value::Tagged(tagged) => yaml_value_to_json(tagged.value),
    }
}

fn yaml_key_to_string(key: serde_yaml::Value) -> Option<String> {
    match key {
        serde_yaml::Value::String(s) => Some(s),
        _ => None,
    }
}

/// Parse or look up semantically parsed frontmatter fields.
///
/// `source` may be a fenced YAML frontmatter string, the `frontmatter_fields`
/// array from `page_index.json`, or a legacy object map.
///
/// When `key` is `None`, returns the normalized array of `{ "key": value }` entries.
/// When `key` is `Some`, returns that root field's semantic value.
#[must_use]
pub fn frontmatter_field(source: &Value, key: Option<&str>) -> Option<Value> {
    let parsed: Vec<Value> = match source {
        Value::String(text) => parse_frontmatter_fields(text)?,
        Value::Array(items) => items.clone(),
        Value::Object(map) => map
            .iter()
            .map(|(field_key, value)| {
                let mut entry = Map::new();
                entry.insert(field_key.clone(), value.clone());
                Value::Object(entry)
            })
            .collect(),
        _ => return None,
    };
    match key {
        None => Some(Value::Array(parsed)),
        Some(field_key) => parsed
            .iter()
            .find_map(|entry| entry.as_object()?.get(field_key))
            .cloned(),
    }
}

/// Parse root-level YAML frontmatter keys into semantic JSON values.
///
/// Returns one `{ "key": value }` object per root YAML key, preserving source order.
/// For example, `description: >-` is folded into a single string value.
#[must_use]
pub fn parse_frontmatter_fields(frontmatter: &str) -> Option<Vec<Value>> {
    let body = frontmatter_yaml_body(frontmatter)?;
    let yaml: serde_yaml::Value = serde_yaml::from_str(&body).ok()?;
    let serde_yaml::Value::Mapping(map) = yaml else {
        return None;
    };
    let mut fields = Vec::new();
    for (key, value) in map {
        if let Some(key) = yaml_key_to_string(key) {
            let mut entry = Map::new();
            entry.insert(key, yaml_value_to_json(value));
            fields.push(Value::Object(entry));
        }
    }
    Some(fields)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn extracts_frontmatter_and_preamble() {
        let md = "---\nname: ctx\ndescription: docs\n---\n\nIntro line\n\n## Section\n\nBody";
        let prefix = extract_skill_prefix(md);
        assert_eq!(
            prefix.frontmatter.as_deref(),
            Some("---\nname: ctx\ndescription: docs\n---")
        );
        assert_eq!(prefix.frontmatter_line_num, Some(1));
        assert_eq!(prefix.preamble.as_deref(), Some("Intro line"));
        assert_eq!(prefix.preamble_line_num, Some(6));
    }

    #[test]
    fn parses_folded_description_semantically() -> Result<(), String> {
        let frontmatter = "---\nname: context7-mcp\ndescription: >-\n  This skill should be used when the user asks about libraries,\n  frameworks, or needs code examples.\n---";
        let fields =
            parse_frontmatter_fields(frontmatter).ok_or_else(|| "parsed fields".to_string())?;
        assert_eq!(fields.len(), 2);
        assert_eq!(
            frontmatter_field(&Value::Array(fields.clone()), Some("name"))
                .and_then(|v| v.as_str().map(str::to_string)),
            Some("context7-mcp".to_string())
        );
        assert_eq!(
            frontmatter_field(&Value::Array(fields), Some("description"))
                .and_then(|v| v.as_str().map(str::to_string)),
            Some("This skill should be used when the user asks about libraries, frameworks, or needs code examples.".to_string())
        );
        Ok(())
    }

    #[test]
    fn parses_arbitrary_root_keys() -> Result<(), String> {
        let frontmatter =
            "---\nname: demo\nversion: 2\nenabled: true\ntags:\n  - rust\n  - yaml\n---";
        let fields =
            parse_frontmatter_fields(frontmatter).ok_or_else(|| "parsed fields".to_string())?;
        let source = Value::Array(fields);
        assert_eq!(
            frontmatter_field(&source, Some("name")).and_then(|v| v.as_str().map(str::to_string)),
            Some("demo".to_string())
        );
        assert_eq!(
            frontmatter_field(&source, Some("version")).and_then(|v| v.as_u64()),
            Some(2)
        );
        assert_eq!(
            frontmatter_field(&source, Some("enabled")).and_then(|v| v.as_bool()),
            Some(true)
        );
        let tags = frontmatter_field(&source, Some("tags"))
            .and_then(|v| v.as_array().cloned())
            .ok_or_else(|| "tags array".to_string())?;
        assert_eq!(tags.len(), 2);
        Ok(())
    }

    #[test]
    fn frontmatter_field_normalizes_legacy_object_json() -> Result<(), String> {
        let legacy = json!({"name": "demo", "version": 2});
        let normalized =
            frontmatter_field(&legacy, None).ok_or_else(|| "normalized fields".to_string())?;
        let array = normalized.as_array().ok_or_else(|| "array".to_string())?;
        assert_eq!(array.len(), 2);
        assert_eq!(
            frontmatter_field(&legacy, Some("name")).and_then(|v| v.as_str().map(str::to_string)),
            Some("demo".to_string())
        );
        Ok(())
    }

    #[test]
    fn ignores_headers_in_code_blocks() {
        let md = "```\n# Not a header\n```\n# Real Header\nBody";
        let (nodes, _) = extract_nodes_from_markdown(md);
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].title, "Real Header");
    }
}
