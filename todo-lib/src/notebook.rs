use crate::error::{Result, TodoError};
use crate::task::Task;
use crate::types::Keyword;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Notebook represents a collection of tasks (single file)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Notebook {
    /// File path of this notebook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,

    /// All top-level tasks in the notebook
    pub tasks: Vec<Task>,

    /// Per-file keyword definitions
    pub keywords: Vec<String>,

    /// Notebook metadata
    pub metadata: NotebookMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotebookMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub created: Option<chrono::DateTime<chrono::Local>>,
    pub modified: Option<chrono::DateTime<chrono::Local>>,
}

impl Default for NotebookMetadata {
    fn default() -> Self {
        NotebookMetadata {
            title: None,
            author: None,
            created: Some(chrono::Local::now()),
            modified: Some(chrono::Local::now()),
        }
    }
}

impl Notebook {
    /// Create a new empty notebook
    pub fn new() -> Self {
        Notebook {
            path: None,
            tasks: Vec::new(),
            keywords: vec![
                "TODO".to_string(),
                "DONE".to_string(),
                "INBOX".to_string(),
                "WAITING".to_string(),
                "SOMEDAY".to_string(),
            ],
            metadata: NotebookMetadata::default(),
        }
    }

    /// Create a notebook from a file path
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let content = fs::read_to_string(path)
            .map_err(|e| TodoError::IoError(e))?;

        let mut notebook = Self::parse(&content)?;
        notebook.path = Some(path.to_path_buf());
        Ok(notebook)
    }

    /// Parse notebook from string content
    pub fn parse(content: &str) -> Result<Self> {
        // This will be implemented in the parser module
        crate::parser::parse_org_mode(content)
    }

    /// Save notebook to file
    pub fn save(&mut self) -> Result<()> {
        if let Some(path) = &self.path {
            self.metadata.modified = Some(chrono::Local::now());
            let content = self.to_org_mode()?;
            fs::write(path, content)
                .map_err(|e| TodoError::IoError(e))?;
            Ok(())
        } else {
            Err(TodoError::InvalidTask(
                "No path specified for notebook".to_string(),
            ))
        }
    }

    /// Save notebook to specific path
    pub fn save_to(&mut self, path: impl AsRef<Path>) -> Result<()> {
        self.path = Some(path.as_ref().to_path_buf());
        self.save()
    }

    /// Convert notebook to Org Mode format
    pub fn to_org_mode(&self) -> Result<String> {
        crate::parser::to_org_mode(self)
    }

    /// Add a task to the notebook
    pub fn add_task(&mut self, task: Task) {
        self.metadata.modified = Some(chrono::Local::now());
        self.tasks.push(task);
    }

    /// Remove task by index
    pub fn remove_task(&mut self, index: usize) -> Option<Task> {
        if index < self.tasks.len() {
            self.metadata.modified = Some(chrono::Local::now());
            Some(self.tasks.remove(index))
        } else {
            None
        }
    }

    /// Get all tasks (including nested) that match a predicate
    pub fn find_all<F>(&self, predicate: F) -> Vec<&Task>
    where
        F: Fn(&Task) -> bool,
    {
        self.tasks
            .iter()
            .flat_map(|task| task.find_all(&predicate))
            .collect()
    }

    /// Find tasks by tag
    pub fn find_by_tag(&self, tag: &str) -> Vec<&Task> {
        self.tasks
            .iter()
            .flat_map(|task| task.find_by_tag(tag))
            .collect()
    }

    /// Find tasks by keyword
    pub fn find_by_keyword(&self, keyword: &Keyword) -> Vec<&Task> {
        self.tasks
            .iter()
            .flat_map(|task| task.find_by_keyword(keyword))
            .collect()
    }

    /// Get all inbox tasks
    pub fn inbox(&self) -> Vec<&Task> {
        self.find_by_keyword(&Keyword::Inbox)
    }

    /// Get all TODO tasks
    pub fn todos(&self) -> Vec<&Task> {
        self.find_by_keyword(&Keyword::Todo)
    }

    /// Get all completed tasks
    pub fn done(&self) -> Vec<&Task> {
        self.find_by_keyword(&Keyword::Done)
    }

    /// Get all waiting tasks
    pub fn waiting(&self) -> Vec<&Task> {
        self.find_by_keyword(&Keyword::Waiting)
    }

    /// Get all someday tasks
    pub fn someday(&self) -> Vec<&Task> {
        self.find_by_keyword(&Keyword::Someday)
    }

    /// Get overdue tasks
    pub fn overdue(&self) -> Vec<&Task> {
        self.find_all(|task| task.is_overdue())
    }

    /// Get tasks scheduled for today
    pub fn scheduled_today(&self) -> Vec<&Task> {
        let today = chrono::Local::now().naive_local().date();
        self.find_all(|task| {
            task.scheduled
                .as_ref()
                .map(|s| s.date == today)
                .unwrap_or(false)
        })
    }

    /// Count total tasks (including nested)
    pub fn count_total(&self) -> usize {
        self.tasks.iter().map(|t| t.count_total()).sum()
    }

    /// Count completed tasks (including nested)
    pub fn count_done(&self) -> usize {
        self.tasks.iter().map(|t| t.count_done()).sum()
    }

    /// Calculate overall completion percentage
    pub fn completion_percentage(&self) -> f64 {
        let total = self.count_total() as f64;
        if total == 0.0 {
            return 0.0;
        }
        (self.count_done() as f64 / total) * 100.0
    }

    /// Merge another notebook into this one
    pub fn merge(&mut self, other: Notebook) -> Result<()> {
        self.metadata.modified = Some(chrono::Local::now());
        crate::merge::merge_notebooks(self, other)
    }

    /// Set notebook title
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.metadata.title = Some(title.into());
        self
    }

    /// Set notebook author
    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.metadata.author = Some(author.into());
        self
    }
}

impl Default for Notebook {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notebook_creation() {
        let notebook = Notebook::new();
        assert_eq!(notebook.tasks.len(), 0);
        assert!(notebook.keywords.contains(&"TODO".to_string()));
    }

    #[test]
    fn test_add_remove_task() {
        let mut notebook = Notebook::new();
        let task = Task::new("Test task");

        notebook.add_task(task.clone());
        assert_eq!(notebook.tasks.len(), 1);

        let removed = notebook.remove_task(0);
        assert!(removed.is_some());
        assert_eq!(notebook.tasks.len(), 0);
    }

    #[test]
    fn test_find_by_keyword() {
        let mut notebook = Notebook::new();

        notebook.add_task(Task::new("Task 1").with_keyword(Keyword::Inbox));
        notebook.add_task(Task::new("Task 2").with_keyword(Keyword::Todo));
        notebook.add_task(Task::new("Task 3").with_keyword(Keyword::Inbox));

        let inbox = notebook.inbox();
        assert_eq!(inbox.len(), 2);
    }

    #[test]
    fn test_completion_stats() {
        let mut notebook = Notebook::new();

        let mut task1 = Task::new("Task 1");
        task1.mark_done();

        notebook.add_task(task1);
        notebook.add_task(Task::new("Task 2"));

        assert_eq!(notebook.count_total(), 2);
        assert_eq!(notebook.count_done(), 1);
        assert_eq!(notebook.completion_percentage(), 50.0);
    }
}
