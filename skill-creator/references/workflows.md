# Workflow Patterns

## Sequential Workflows

For complex tasks, break operations into clear, sequential steps. It is often helpful to give Claude an overview of the process towards the beginning of SKILL.md:

```markdown
Filling a PDF form involves these steps:

1. Analyze the form (run analyze_form.py)
2. Create field mapping (edit fields.json)
3. Validate mapping (run validate_fields.py)
4. Fill the form (run fill_form.py)
5. Verify output (run verify_output.py)
```

## Conditional Workflows

For tasks with branching logic, guide Claude through decision points:

```markdown
1. Determine the modification type:
   **Creating new content?** → Follow "Creation workflow" below
   **Editing existing content?** → Follow "Editing workflow" below

2. Creation workflow: [steps]
3. Editing workflow: [steps]
```

## Plan-With-Files TODO Workflow (Recommended for Skill Creation)

Use a single TODO file as the execution contract for multi-file skill authoring:

1. Create `./output/skill-creation.todo.md` from `references/plan-with-files.todo-template.md`
2. Build an internal understanding record from task history evidence
3. Present the understanding to the user in natural conversational Chinese (not form labels), then wait for confirmation (`[x] confirmed`)
4. Execute tasks T0..T6 in order, updating status on each transition
5. Only package after prerequisite tasks are completed

This pattern makes progress auditable and prevents skipping core quality gates.
