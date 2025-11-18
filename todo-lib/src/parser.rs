use crate::error::{Result, TodoError};
use crate::notebook::{Notebook, NotebookMetadata};
use crate::task::Task;
use crate::types::{Keyword, PeriodicDateTime, Priority};
use chrono::NaiveDate;
use regex::Regex;
use std::collections::HashMap;

/// Parse Org Mode content into a Notebook
pub fn parse_org_mode(content: &str) -> Result<Notebook> {
    let mut notebook = Notebook::new();
    let lines: Vec<&str> = content.lines().collect();

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim_start();

        // Parse metadata
        if line.starts_with("#+TITLE:") {
            notebook.metadata.title = Some(line[8..].trim().to_string());
            i += 1;
            continue;
        }

        if line.starts_with("#+AUTHOR:") {
            notebook.metadata.author = Some(line[9..].trim().to_string());
            i += 1;
            continue;
        }

        // Parse tasks (headlines starting with *)
        if line.starts_with('*') {
            let (task, next_i) = parse_task(&lines, i)?;
            notebook.tasks.push(task);
            i = next_i;
        } else {
            i += 1;
        }
    }

    Ok(notebook)
}

/// Parse a single task and its children
fn parse_task(lines: &[&str], start: usize) -> Result<(Task, usize)> {
    let line = lines[start];
    let level = line.chars().take_while(|&c| c == '*').count();

    // Parse headline components
    let headline = &line[level..].trim();
    let (keyword, rest) = parse_keyword(headline);
    let (priority, rest) = parse_priority(rest);
    let (title, tags) = parse_title_and_tags(rest);

    let mut task = Task::new(title)
        .with_keyword(keyword)
        .with_tags(tags);

    if let Some(p) = priority {
        task = task.with_priority(p);
    }

    // Parse task body and properties
    let mut i = start + 1;
    let mut body_lines = Vec::new();
    let mut in_properties = false;

    while i < lines.len() {
        let current_line = lines[i];
        let current_level = current_line.chars().take_while(|&c| c == '*').count();

        // Check if we've hit another task at same or higher level
        if current_line.starts_with('*') && current_level <= level {
            break;
        }

        // Check if this is a child task
        if current_line.starts_with('*') && current_level == level + 1 {
            let (child, next_i) = parse_task(lines, i)?;
            task.add_child(child);
            i = next_i;
            continue;
        }

        // Parse properties drawer
        if current_line.trim() == ":PROPERTIES:" {
            in_properties = true;
            i += 1;
            continue;
        }

        if current_line.trim() == ":END:" {
            in_properties = false;
            i += 1;
            continue;
        }

        if in_properties {
            if let Some((key, value)) = parse_property(current_line) {
                task.set_property(key, value);
            }
            i += 1;
            continue;
        }

        // Parse scheduling and deadline
        if current_line.contains("SCHEDULED:") {
            if let Some(datetime) = parse_datetime(current_line, "SCHEDULED:") {
                task.scheduled = Some(datetime);
            }
        }

        if current_line.contains("DEADLINE:") {
            if let Some(datetime) = parse_datetime(current_line, "DEADLINE:") {
                task.deadline = Some(datetime);
            }
        }

        // Add to body if not empty
        if !current_line.trim().is_empty()
            && !current_line.contains("SCHEDULED:")
            && !current_line.contains("DEADLINE:")
        {
            body_lines.push(current_line);
        }

        i += 1;
    }

    if !body_lines.is_empty() {
        task.body = Some(body_lines.join("\n"));
    }

    Ok((task, i))
}

/// Parse keyword from headline
fn parse_keyword(headline: &str) -> (Keyword, &str) {
    let parts: Vec<&str> = headline.splitn(2, ' ').collect();
    if parts.len() < 2 {
        return (Keyword::Todo, headline);
    }

    let potential_keyword = parts[0].trim();
    let keywords = ["TODO", "DONE", "INBOX", "WAITING", "SOMEDAY"];

    if keywords.contains(&potential_keyword) {
        (Keyword::from_str(potential_keyword), parts[1].trim())
    } else {
        (Keyword::Todo, headline)
    }
}

/// Parse priority from headline
fn parse_priority(headline: &str) -> (Option<Priority>, &str) {
    let re = Regex::new(r"^\[#([A-Z])\]\s+(.*)$").unwrap();

    if let Some(caps) = re.captures(headline) {
        if let Some(letter) = caps.get(1) {
            let priority = Priority::from_letter(letter.as_str().chars().next().unwrap());
            let rest = caps.get(2).map(|m| m.as_str()).unwrap_or("");
            return (priority, rest);
        }
    }

    (None, headline)
}

/// Parse title and tags from headline
fn parse_title_and_tags(headline: &str) -> (String, Vec<String>) {
    let re = Regex::new(r"^(.*?)\s+:([\w:]+):$").unwrap();

    if let Some(caps) = re.captures(headline) {
        let title = caps.get(1).map(|m| m.as_str().trim()).unwrap_or("");
        let tags_str = caps.get(2).map(|m| m.as_str()).unwrap_or("");
        let tags: Vec<String> = tags_str
            .split(':')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        (title.to_string(), tags)
    } else {
        (headline.to_string(), Vec::new())
    }
}

/// Parse property line
fn parse_property(line: &str) -> Option<(String, String)> {
    let re = Regex::new(r"^\s*:(\w+):\s+(.+)$").unwrap();

    re.captures(line).and_then(|caps| {
        let key = caps.get(1)?.as_str().to_string();
        let value = caps.get(2)?.as_str().to_string();
        Some((key, value))
    })
}

/// Parse datetime from org mode format
fn parse_datetime(line: &str, prefix: &str) -> Option<PeriodicDateTime> {
    let re = Regex::new(r"<(\d{4})-(\d{2})-(\d{2}).*?>").unwrap();

    if let Some(caps) = re.captures(line) {
        let year = caps.get(1)?.as_str().parse().ok()?;
        let month = caps.get(2)?.as_str().parse().ok()?;
        let day = caps.get(3)?.as_str().parse().ok()?;

        let date = NaiveDate::from_ymd_opt(year, month, day)?;
        Some(PeriodicDateTime::new(date, None))
    } else {
        None
    }
}

/// Convert Notebook to Org Mode format
pub fn to_org_mode(notebook: &Notebook) -> Result<String> {
    let mut output = String::new();

    // Write metadata
    if let Some(title) = &notebook.metadata.title {
        output.push_str(&format!("#+TITLE: {}\n", title));
    }

    if let Some(author) = &notebook.metadata.author {
        output.push_str(&format!("#+AUTHOR: {}\n", author));
    }

    if !output.is_empty() {
        output.push('\n');
    }

    // Write tasks
    for task in &notebook.tasks {
        output.push_str(&task_to_org_mode(task, 1)?);
    }

    Ok(output)
}

/// Convert Task to Org Mode format
fn task_to_org_mode(task: &Task, level: usize) -> Result<String> {
    let mut output = String::new();

    // Write headline
    let stars = "*".repeat(level);
    output.push_str(&stars);
    output.push(' ');

    // Write keyword
    output.push_str(task.keyword.as_str());
    output.push(' ');

    // Write priority
    if let Some(priority) = &task.priority {
        output.push_str(&format!("[#{}] ", priority.to_letter()));
    }

    // Write title
    output.push_str(&task.title);

    // Write tags
    if !task.tags.is_empty() {
        output.push_str(" :");
        output.push_str(&task.tags.join(":"));
        output.push(':');
    }

    output.push('\n');

    // Write scheduling
    if let Some(scheduled) = &task.scheduled {
        output.push_str(&format!("SCHEDULED: <{}>\n", scheduled.date));
    }

    // Write deadline
    if let Some(deadline) = &task.deadline {
        output.push_str(&format!("DEADLINE: <{}>\n", deadline.date));
    }

    // Write properties
    if !task.properties.is_empty() || task.id.is_some() {
        output.push_str(":PROPERTIES:\n");

        if let Some(id) = &task.id {
            output.push_str(&format!(":ID: {}\n", id));
        }

        for (key, value) in &task.properties {
            output.push_str(&format!(":{}:  {}\n", key, value));
        }

        output.push_str(":END:\n");
    }

    // Write body
    if let Some(body) = &task.body {
        output.push_str(body);
        output.push('\n');
    }

    // Write children
    for child in &task.children {
        output.push_str(&task_to_org_mode(child, level + 1)?);
    }

    output.push('\n');

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_keyword() {
        let (keyword, rest) = parse_keyword("TODO Buy milk");
        assert_eq!(keyword, Keyword::Todo);
        assert_eq!(rest, "Buy milk");
    }

    #[test]
    fn test_parse_priority() {
        let (priority, rest) = parse_priority("[#A] Important task");
        assert_eq!(priority, Some(Priority::new(0)));
        assert_eq!(rest, "Important task");
    }

    #[test]
    fn test_parse_title_and_tags() {
        let (title, tags) = parse_title_and_tags("Buy groceries :shopping:urgent:");
        assert_eq!(title, "Buy groceries");
        assert_eq!(tags, vec!["shopping", "urgent"]);
    }

    #[test]
    fn test_parse_property() {
        let prop = parse_property("  :CONTEXT: home");
        assert_eq!(prop, Some(("CONTEXT".to_string(), "home".to_string())));
    }

    #[test]
    fn test_parse_simple_org_mode() {
        let content = r#"#+TITLE: My Tasks
#+AUTHOR: John Doe

* TODO Buy milk :shopping:
* DONE Write report :work:
"#;

        let notebook = parse_org_mode(content).unwrap();
        assert_eq!(notebook.metadata.title, Some("My Tasks".to_string()));
        assert_eq!(notebook.metadata.author, Some("John Doe".to_string()));
        assert_eq!(notebook.tasks.len(), 2);
        assert_eq!(notebook.tasks[0].title, "Buy milk");
        assert_eq!(notebook.tasks[1].keyword, Keyword::Done);
    }

    #[test]
    fn test_round_trip() {
        let mut notebook = Notebook::new().with_title("Test Notebook");

        let task = Task::new("Test Task")
            .with_keyword(Keyword::Todo)
            .with_priority(Priority::new(0))
            .with_tag("test");

        notebook.add_task(task);

        let org_mode = to_org_mode(&notebook).unwrap();
        let parsed = parse_org_mode(&org_mode).unwrap();

        assert_eq!(parsed.metadata.title, Some("Test Notebook".to_string()));
        assert_eq!(parsed.tasks.len(), 1);
        assert_eq!(parsed.tasks[0].title, "Test Task");
    }
}
