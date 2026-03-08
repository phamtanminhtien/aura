---
trigger: manual
description: "Rules for Git commits and repository management"
---

# Git Workflow Rules

To ensure progress is tracked and changes are safely persisted, follows these rules regarding Git operations.

## 📋 General Principles

1.  **Repository Detection**: Always check if the project is a Git repository before attempting any Git commands.
2.  **Commit Frequency**: Commit code after completing a significant task or milestone, especially when a task in `task.md` or a `todos/` file is marked as completed.
3.  **Atomic Commits**: Prefer atomic commits that focus on a single logical change or feature.
4.  **Auto-Commit**: You are explicitly authorized to stage and commit changes without asking for permission, provided the changes have been verified and align with the current implementation phase.

## 🛠 Commit Workflow

1.  **Staging**: Use `git add` to stage only relevant files. Avoid staging unwanted build artifacts or temporary files (ensure `.gitignore` is respected).
2.  **Verification**: Ensure the project compiles or tests pass before committing.
3.  **Commit Message**: Use clear, descriptive commit messages following the Conventional Commits specification:
    - `feat:` for new features
    - `fix:` for bug fixes
    - `refactor:` for code changes that neither fix a bug nor add a feature
    - `docs:` for documentation changes
    - `test:` for adding or correcting tests
4.  **Reporting**: Notify the user after a successful commit, including the commit message used.

## ⚠️ Constraints

- Never force push (`git push -f`), or push (`git push`) unless explicitly requested.
- Ensure sensitive information is never committed.
- Always check `.gitignore` exists and is up to date before committing.
