# Skill Creation TODO (Plan-With-Files)

> Copy this file to `./output/skill-creation.todo.md` before implementation.
> This file is the single source of truth for progress.

## 0. Context

- Request source:
- Task history scope:
- Enabled skills observed:
- Final status of source task(s):

## 1. Understanding Record (internal, must be confirmed before implementation)

> Note: This section is for internal tracking.
> When talking to the user, use natural paragraph-style Chinese and do not mirror these fields as a rigid template.

- User goal:
- Context and audience:
- Reusable successful methods from history:
- User preferences and hard constraints:
- Recommended skill granularity:
- Proposed skill id (kebab-case):
- Proposed display name:
- User confirmation status: `[ ] pending` / `[x] confirmed`

## 2. Task Checklist

Legend:

- `[ ]` not started
- `[-]` in progress
- `[x]` completed
- `[!]` blocked

### T0 - Extract and summarize intent from task history

- Status: `[ ]`
- Files:
  - `./output/skill-creation.todo.md`
- Commands/evidence:
  - (record message/history source)
- Completion criteria:
  - Understanding record is filled with concrete evidence
  - A user-facing recap can be stated in natural conversational Chinese

### T1 - Get user confirmation on understanding recap

- Status: `[ ]`
- Files:
  - `./output/skill-creation.todo.md`
- Commands/evidence:
  - (record user confirmation message)
- Completion criteria:
  - `User confirmation status` is `[x] confirmed`

### T2 - Initialize skill skeleton

- Status: `[ ]`
- Files:
  - `<skill-root>/SKILL.md`
  - `<skill-root>/scripts/`
  - `<skill-root>/references/`
  - `<skill-root>/assets/`
- Commands/evidence:
  - `scripts/init_skill.py <skill-id> --path <output-dir>`
- Completion criteria:
  - New skill folder exists with required structure

### T3 - Implement and edit skill files

- Status: `[ ]`
- Files:
  - `<skill-root>/SKILL.md`
  - `<skill-root>/scripts/*`
  - `<skill-root>/references/*`
  - `<skill-root>/assets/*` (if needed)
- Commands/evidence:
  - (record key file edits)
- Completion criteria:
  - Skill content reflects confirmed understanding summary

### T4 - Validate skill

- Status: `[ ]`
- Files:
  - `<skill-root>/SKILL.md`
- Commands/evidence:
  - `scripts/quick_validate.py <skill-root>`
- Completion criteria:
  - Validator returns success

### T5 - Package zip artifact

- Status: `[ ]`
- Files:
  - `./output/<skill-id>.zip`
- Commands/evidence:
  - `scripts/package_skill.py <skill-root> ./output --todo ./output/skill-creation.todo.md`
- Completion criteria:
  - Zip file exists with `.zip` suffix and valid structure

### T6 - Verify delivery path and artifact info

- Status: `[ ]`
- Files:
  - `./output/<skill-id>.zip`
- Commands/evidence:
  - `ls -la ./output`
  - `unzip -l ./output/<skill-id>.zip`
- Completion criteria:
  - Artifact path and filename are shared clearly with user

## 3. Progress Log

- 2026-..-.. ..:..:.. :
