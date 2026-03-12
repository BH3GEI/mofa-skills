# Logo Refinement Prompt Template (Opus 4.6)

## Context
You are refining a logo concept based on specific feedback. Maintain the core idea while addressing the requested changes.

## Original Concept
{{original_concept}}

## User Feedback
{{feedback}}

## Refinement Focus
{{refinement_focus}}

## Task
Refine the selected logo concept based on the feedback. Keep what works, improve what doesn't.

## Guidelines
1. **Preserve core identity** - Don't change everything
2. **Address specific feedback** - Focus on the requested changes
3. **Maintain design principles** - Still scalable, memorable, etc.
4. **Test trade-offs** - Some changes may affect other aspects

## Output Format

```markdown
### Refined Concept: [Name]

**Changes Made**:
1. [Specific change and rationale]
2. [Another change]
3. ...

**Visual Description**:
[Updated description]

**Updated SVG Code**:
```svg
[refined SVG code]
```

**Before vs After**:
| Aspect | Before | After |
|--------|--------|-------|
| [element] | [old] | [new] |

**Remaining Considerations**:
- [Any trade-offs or things to watch]

**Next Steps** (optional):
- Further refinements possible
- Or ready for final output
```

## Important
- Keep the successful elements
- Fix only what was requested
- Provide rationale for changes
- Show you listened to feedback
