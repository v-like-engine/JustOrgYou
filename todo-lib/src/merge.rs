use crate::error::{Result, TodoError};
use crate::notebook::Notebook;
use crate::task::Task;
use std::collections::HashMap;

/// Strategy for handling merge conflicts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeStrategy {
    /// Take the local version
    PreferLocal,
    /// Take the remote version
    PreferRemote,
    /// Keep both as separate tasks
    KeepBoth,
    /// Merge properties intelligently
    Smart,
}

/// Merge two notebooks together
pub fn merge_notebooks(target: &mut Notebook, source: Notebook) -> Result<()> {
    merge_notebooks_with_strategy(target, source, MergeStrategy::Smart)
}

/// Merge two notebooks with a specific strategy
pub fn merge_notebooks_with_strategy(
    target: &mut Notebook,
    source: Notebook,
    strategy: MergeStrategy,
) -> Result<()> {
    // Build ID maps for efficient lookup
    let mut target_map: HashMap<String, &mut Task> = HashMap::new();

    for task in &mut target.tasks {
        collect_tasks_by_id(task, &mut target_map);
    }

    // Merge source tasks
    for source_task in source.tasks {
        merge_task_into_notebook(target, source_task, &target_map, strategy)?;
    }

    // Merge keywords
    for keyword in source.keywords {
        if !target.keywords.contains(&keyword) {
            target.keywords.push(keyword);
        }
    }

    Ok(())
}

/// Recursively collect tasks by ID
fn collect_tasks_by_id<'a>(task: &'a mut Task, map: &mut HashMap<String, &'a mut Task>) {
    if let Some(id) = &task.id {
        map.insert(id.clone(), task);
    }

    for child in &mut task.children {
        collect_tasks_by_id(child, map);
    }
}

/// Merge a source task into the target notebook
fn merge_task_into_notebook(
    target: &mut Notebook,
    source_task: Task,
    target_map: &HashMap<String, &mut Task>,
    strategy: MergeStrategy,
) -> Result<()> {
    // If source task has an ID, check if it exists in target
    if let Some(id) = &source_task.id {
        if target_map.contains_key(id) {
            // Task exists, merge based on strategy
            match strategy {
                MergeStrategy::PreferRemote => {
                    // Replace existing task
                    replace_task_by_id(target, id, source_task);
                }
                MergeStrategy::KeepBoth => {
                    // Add as new task with new ID
                    let mut new_task = source_task.clone();
                    new_task.id = Some(uuid::Uuid::new_v4().to_string());
                    target.add_task(new_task);
                }
                MergeStrategy::Smart | MergeStrategy::PreferLocal => {
                    // Smart merge - update properties but keep local structure
                    // For now, prefer local (no action needed)
                    // In future, can implement more sophisticated merging
                }
            }
        } else {
            // New task, add it
            target.add_task(source_task);
        }
    } else {
        // No ID, always add as new task
        target.add_task(source_task);
    }

    Ok(())
}

/// Replace a task by ID in the notebook
fn replace_task_by_id(notebook: &mut Notebook, id: &str, new_task: Task) {
    for (i, task) in notebook.tasks.iter_mut().enumerate() {
        if task.id.as_ref().map(|s| s.as_str()) == Some(id) {
            notebook.tasks[i] = new_task;
            return;
        }

        if replace_task_in_children(task, id, new_task.clone()) {
            return;
        }
    }
}

/// Recursively replace a task in children
fn replace_task_in_children(parent: &mut Task, id: &str, new_task: Task) -> bool {
    for (i, child) in parent.children.iter_mut().enumerate() {
        if child.id.as_ref().map(|s| s.as_str()) == Some(id) {
            parent.children[i] = new_task;
            return true;
        }

        if replace_task_in_children(child, id, new_task.clone()) {
            return true;
        }
    }

    false
}

/// Merge two tasks with conflict detection
pub fn merge_tasks(
    local: &Task,
    remote: &Task,
    strategy: MergeStrategy,
) -> Result<Task> {
    // Check if tasks are the same
    if local.id.is_some() && local.id == remote.id {
        Ok(merge_tasks_smart(local, remote))
    } else {
        Err(TodoError::MergeConflict(
            "Tasks have different IDs or no IDs".to_string(),
        ))
    }
}

/// Smart merge of two tasks
fn merge_tasks_smart(local: &Task, remote: &Task) -> Task {
    let mut merged = local.clone();

    // Merge title - prefer non-empty
    if local.title.is_empty() && !remote.title.is_empty() {
        merged.title = remote.title.clone();
    }

    // Merge body - prefer longer
    match (&local.body, &remote.body) {
        (None, Some(b)) => merged.body = Some(b.clone()),
        (Some(a), Some(b)) if b.len() > a.len() => merged.body = Some(b.clone()),
        _ => {}
    }

    // Merge keyword - prefer DONE over TODO
    if remote.keyword.is_done() && !local.keyword.is_done() {
        merged.keyword = remote.keyword.clone();
    }

    // Merge priority - prefer higher priority (lower number)
    match (&local.priority, &remote.priority) {
        (None, Some(p)) => merged.priority = Some(*p),
        (Some(a), Some(b)) if b < a => merged.priority = Some(*b),
        _ => {}
    }

    // Merge tags - union
    for tag in &remote.tags {
        if !merged.tags.contains(tag) {
            merged.tags.push(tag.clone());
        }
    }

    // Merge properties - union
    for (key, value) in &remote.properties {
        merged.properties.entry(key.clone()).or_insert(value.clone());
    }

    // Merge scheduled/deadline - prefer remote if local is None
    if local.scheduled.is_none() && remote.scheduled.is_some() {
        merged.scheduled = remote.scheduled.clone();
    }

    if local.deadline.is_none() && remote.deadline.is_some() {
        merged.deadline = remote.deadline.clone();
    }

    // Merge children - this is complex, for now just keep local
    // Future: implement recursive merge of children by ID

    merged
}

/// Three-way merge using a common ancestor
pub fn three_way_merge(
    base: &Task,
    local: &Task,
    remote: &Task,
) -> Result<Task> {
    let mut merged = base.clone();

    // Merge title
    if local.title != base.title {
        merged.title = local.title.clone();
    } else if remote.title != base.title {
        merged.title = remote.title.clone();
    }

    // Merge keyword
    if local.keyword != base.keyword {
        merged.keyword = local.keyword.clone();
    } else if remote.keyword != base.keyword {
        merged.keyword = remote.keyword.clone();
    }

    // Detect conflicts
    if local.keyword != base.keyword
        && remote.keyword != base.keyword
        && local.keyword != remote.keyword
    {
        return Err(TodoError::MergeConflict(
            "Conflicting keyword changes".to_string(),
        ));
    }

    // Merge priority
    match (&base.priority, &local.priority, &remote.priority) {
        (_, lp, rp) if lp != rp => {
            // Prefer higher priority
            merged.priority = match (lp, rp) {
                (Some(a), Some(b)) => Some(if a < b { *a } else { *b }),
                (Some(p), None) | (None, Some(p)) => Some(*p),
                (None, None) => None,
            };
        }
        (_, lp, _) => merged.priority = *lp,
    }

    // Merge tags - union of all changes
    merged.tags = base.tags.clone();
    for tag in &local.tags {
        if !merged.tags.contains(tag) {
            merged.tags.push(tag.clone());
        }
    }
    for tag in &remote.tags {
        if !merged.tags.contains(tag) {
            merged.tags.push(tag.clone());
        }
    }

    Ok(merged)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Keyword, Priority};

    #[test]
    fn test_smart_merge_tasks() {
        let local = Task::new("Task A")
            .with_id("123")
            .with_keyword(Keyword::Todo)
            .with_tag("work");

        let remote = Task::new("Task A")
            .with_id("123")
            .with_keyword(Keyword::Done)
            .with_tag("urgent");

        let merged = merge_tasks_smart(&local, &remote);

        assert_eq!(merged.keyword, Keyword::Done);
        assert!(merged.tags.contains(&"work".to_string()));
        assert!(merged.tags.contains(&"urgent".to_string()));
    }

    #[test]
    fn test_merge_notebooks() {
        let mut target = Notebook::new();
        target.add_task(Task::new("Task 1").with_id("1"));

        let mut source = Notebook::new();
        source.add_task(Task::new("Task 2").with_id("2"));

        merge_notebooks(&mut target, source).unwrap();

        assert_eq!(target.tasks.len(), 2);
    }

    #[test]
    fn test_three_way_merge_no_conflict() {
        let base = Task::new("Base").with_id("1").with_keyword(Keyword::Todo);

        let local = Task::new("Base")
            .with_id("1")
            .with_keyword(Keyword::Todo)
            .with_tag("work");

        let remote = Task::new("Base")
            .with_id("1")
            .with_keyword(Keyword::Done);

        let merged = three_way_merge(&base, &local, &remote).unwrap();

        assert_eq!(merged.keyword, Keyword::Done);
        assert!(merged.tags.contains(&"work".to_string()));
    }

    #[test]
    fn test_three_way_merge_conflict() {
        let base = Task::new("Base").with_id("1").with_keyword(Keyword::Todo);

        let local = Task::new("Base")
            .with_id("1")
            .with_keyword(Keyword::Done);

        let remote = Task::new("Base")
            .with_id("1")
            .with_keyword(Keyword::Waiting);

        let result = three_way_merge(&base, &local, &remote);
        assert!(result.is_err());
    }
}
