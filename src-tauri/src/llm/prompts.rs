//! Prompt templates for LLM interactions
//!
//! Includes project context injection for more relevant responses.

use crate::models::{Project, Task};

/// Build system prompt with project context
///
/// Injects project name, description, and task list into the system prompt
/// to provide relevant context for the LLM.
pub fn build_system_prompt(project: &Project, tasks: &[Task]) -> String {
    let mut prompt = String::new();

    // Project overview
    prompt.push_str("## Project Context\n\n");
    prompt.push_str(&format!("**Project Name:** {}\n", project.name));

    if let Some(desc) = &project.description {
        prompt.push_str(&format!("**Description:** {}\n", desc));
    }

    // Project status
    let status_str = match project.status {
        0 => "Not Started",
        1 => "In Progress",
        2 => "Completed",
        3 => "Archived",
        _ => "Unknown",
    };
    prompt.push_str(&format!("**Status:** {}\n", status_str));

    // Date information
    if let Some(start) = &project.start_date {
        prompt.push_str(&format!("**Start Date:** {}\n", start.format("%Y-%m-%d")));
    }
    if let Some(end) = &project.end_date {
        prompt.push_str(&format!("**End Date:** {}\n", end.format("%Y-%m-%d")));
    }

    // Task list
    if !tasks.is_empty() {
        prompt.push_str("\n## Tasks\n\n");
        for task in tasks {
            let status = match task.status {
                0 => "Todo",
                1 => "In Progress",
                2 => "Done",
                _ => "Unknown",
            };
            let priority = match task.priority {
                0 => "Lowest",
                1 => "Low",
                2 => "Medium",
                3 => "High",
                4 => "Highest",
                _ => "Unknown",
            };

            prompt.push_str(&format!(
                "- **{}** [{}] [{}]",
                task.title, status, priority
            ));

            if let Some(desc) = &task.description {
                prompt.push_str(&format!(": {}", desc));
            }
            prompt.push('\n');
        }
    }

    // Add instructions
    prompt.push_str(
        "\n## Instructions\n\n\
        You are a project management assistant. Use the project context above \
        to provide relevant help. Be concise and actionable in your responses. \
        When suggesting tasks or milestones, consider the project status and timeline.",
    );

    prompt
}

/// Build a simple greeting prompt
pub fn build_greeting_prompt() -> String {
    String::from(
        "You are a helpful project management assistant. \
        You help users manage projects, tasks, documents, and milestones. \
        Provide clear, concise, and actionable advice.",
    )
}

/// Build prompt for document analysis
pub fn build_document_prompt(document_title: &str, content: &str) -> String {
    format!(
        "## Document: {}\n\n{}\n\n\
        Please analyze this document and provide:\n\
        1. A brief summary\n\
        2. Key points or action items\n\
        3. Any suggestions for improvement",
        document_title, content
    )
}
