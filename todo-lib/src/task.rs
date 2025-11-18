use crate::types::{Keyword, PeriodicDateTime, Priority};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Core Task data structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Task {
    /// Single line describing the task
    pub title: String,

    /// Multi-line description with details
    pub body: Option<String>,

    /// Child tasks in tree hierarchy
    pub children: Vec<Task>,

    /// Priority indicator (0 = highest)
    pub priority: Option<Priority>,

    /// Task state (TODO, DONE, INBOX, etc.)
    pub keyword: Keyword,

    /// Tags for categorization and search
    pub tags: Vec<String>,

    /// When the task is scheduled to be done
    pub scheduled: Option<PeriodicDateTime>,

    /// Task deadline
    pub deadline: Option<PeriodicDateTime>,

    /// Additional custom properties
    pub properties: HashMap<String, String>,

    /// Unique identifier for the task
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl Task {
    /// Create a new task with minimal information
    pub fn new(title: impl Into<String>) -> Self {
        Task {
            title: title.into(),
            body: None,
            children: Vec::new(),
            priority: None,
            keyword: Keyword::Todo,
            tags: Vec::new(),
            scheduled: None,
            deadline: None,
            properties: HashMap::new(),
            id: None,
        }
    }

    /// Builder pattern methods
    pub fn with_body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = Some(priority);
        self
    }

    pub fn with_keyword(mut self, keyword: Keyword) -> Self {
        self.keyword = keyword;
        self
    }

    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn with_scheduled(mut self, scheduled: PeriodicDateTime) -> Self {
        self.scheduled = Some(scheduled);
        self
    }

    pub fn with_deadline(mut self, deadline: PeriodicDateTime) -> Self {
        self.deadline = Some(deadline);
        self
    }

    pub fn with_child(mut self, child: Task) -> Self {
        self.children.push(child);
        self
    }

    pub fn with_property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.properties.insert(key.into(), value.into());
        self
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Check if task is completed
    pub fn is_done(&self) -> bool {
        self.keyword.is_done()
    }

    /// Mark task as done
    pub fn mark_done(&mut self) {
        self.keyword = Keyword::Done;
    }

    /// Check if task is overdue
    pub fn is_overdue(&self) -> bool {
        self.deadline
            .as_ref()
            .map(|d| d.is_past())
            .unwrap_or(false)
    }

    /// Get all tasks (including nested) that match a predicate
    pub fn find_all<F>(&self, predicate: &F) -> Vec<&Task>
    where
        F: Fn(&Task) -> bool,
    {
        let mut results = Vec::new();

        if predicate(self) {
            results.push(self);
        }

        for child in &self.children {
            results.extend(child.find_all(predicate));
        }

        results
    }

    /// Get all tasks with a specific tag
    pub fn find_by_tag(&self, tag: &str) -> Vec<&Task> {
        self.find_all(&|task| task.tags.iter().any(|t| t == tag))
    }

    /// Get all tasks with a specific keyword
    pub fn find_by_keyword(&self, keyword: &Keyword) -> Vec<&Task> {
        self.find_all(&|task| &task.keyword == keyword)
    }

    /// Count total tasks (including children)
    pub fn count_total(&self) -> usize {
        1 + self.children.iter().map(|c| c.count_total()).sum::<usize>()
    }

    /// Count completed tasks (including children)
    pub fn count_done(&self) -> usize {
        let done = if self.is_done() { 1 } else { 0 };
        done + self
            .children
            .iter()
            .map(|c| c.count_done())
            .sum::<usize>()
    }

    /// Calculate completion percentage
    pub fn completion_percentage(&self) -> f64 {
        let total = self.count_total() as f64;
        if total == 0.0 {
            return 0.0;
        }
        (self.count_done() as f64 / total) * 100.0
    }

    /// Add a child task
    pub fn add_child(&mut self, child: Task) {
        self.children.push(child);
    }

    /// Remove child task by index
    pub fn remove_child(&mut self, index: usize) -> Option<Task> {
        if index < self.children.len() {
            Some(self.children.remove(index))
        } else {
            None
        }
    }

    /// Get property value
    pub fn get_property(&self, key: &str) -> Option<&String> {
        self.properties.get(key)
    }

    /// Set property value
    pub fn set_property(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.properties.insert(key.into(), value.into());
    }

    /// Remove property
    pub fn remove_property(&mut self, key: &str) -> Option<String> {
        self.properties.remove(key)
    }

    /// Generate unique ID if not present
    pub fn ensure_id(&mut self) {
        if self.id.is_none() {
            self.id = Some(uuid::Uuid::new_v4().to_string());
        }
    }
}

impl Default for Task {
    fn default() -> Self {
        Task::new("New Task")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Priority;

    #[test]
    fn test_task_creation() {
        let task = Task::new("Buy milk")
            .with_priority(Priority::new(0))
            .with_tag("shopping")
            .with_keyword(Keyword::Todo);

        assert_eq!(task.title, "Buy milk");
        assert_eq!(task.priority, Some(Priority::new(0)));
        assert_eq!(task.tags, vec!["shopping"]);
        assert_eq!(task.keyword, Keyword::Todo);
    }

    #[test]
    fn test_task_completion() {
        let mut task = Task::new("Test task");
        assert!(!task.is_done());

        task.mark_done();
        assert!(task.is_done());
    }

    #[test]
    fn test_task_hierarchy() {
        let mut parent = Task::new("Parent task");
        let child1 = Task::new("Child 1");
        let child2 = Task::new("Child 2");

        parent.add_child(child1);
        parent.add_child(child2);

        assert_eq!(parent.children.len(), 2);
        assert_eq!(parent.count_total(), 3);
    }

    #[test]
    fn test_find_by_tag() {
        let mut parent = Task::new("Parent").with_tag("work");
        parent.add_child(Task::new("Child 1").with_tag("urgent"));
        parent.add_child(Task::new("Child 2").with_tag("work"));

        let work_tasks = parent.find_by_tag("work");
        assert_eq!(work_tasks.len(), 2);
    }

    #[test]
    fn test_completion_percentage() {
        let mut parent = Task::new("Parent");
        let mut child1 = Task::new("Child 1");
        child1.mark_done();
        let child2 = Task::new("Child 2");

        parent.add_child(child1);
        parent.add_child(child2);

        // 1 out of 3 tasks done
        assert!((parent.completion_percentage() - 33.33).abs() < 0.1);
    }

    #[test]
    fn test_properties() {
        let mut task = Task::new("Test");
        task.set_property("context", "home");
        task.set_property("energy", "high");

        assert_eq!(task.get_property("context"), Some(&"home".to_string()));
        assert_eq!(task.properties.len(), 2);

        task.remove_property("energy");
        assert_eq!(task.properties.len(), 1);
    }
}
