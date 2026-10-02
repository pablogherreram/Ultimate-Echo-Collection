# Troubleshooting and Lessons Learned

## Purpose

This document records real failure modes encountered while creating, compiling, reorganizing, and validating the first functional Echo Fighter project.

Consult this document before repeating a failed approach.

## 1. Preserve a Known-Good State

### Mistake

Continuing to edit the only functional copy of a mod or plugin.

### Prevention

- Keep the original mod unchanged.
- Work in a separate copy.
- Preserve the first successful hardware-tested package as a private golden image.
- Never overwrite the golden image with presentation fixes.

## 2. Separate Source, Assets, and Artifacts

### Mistake

Treating the source repository, complete mod package, and compiled NRO as one thing.

### Correct model

```text
Source repository
→ compiles plugin.nro

Local mod package
→ contains assets and plugin.nro

Private golden image
→ preserves the full tested state
```

Do not commit complete third-party assets or private backups to normal source history.

## 3. Do Not Duplicate Existing Styles

### Mistake

Assuming a mod needs another block of color slots even though the original package already contains multiple complete styles.

### Prevention

Inspect the complete tree first. Distinct `c20-c28` resources can represent nine legitimate styles, not redundant files.

Only create the marker files needed for the existing intended slots.

## 4. Marker Problems

### Symptoms

- no new character entry;
- fewer styles than expected;
- later slots ignored;
- plugin apparently compiles but does not detect the mod.

### Checks

- exact marker filename;
- exact case;
- correct base fighter directory;
- correct path under `model/body/cXX`;
- continuous slot sequence;
- no accidental marker outside the intended range;
- marker count matches the expected style count.

Example:

```text
fighter/koopa/model/body/c20/koopadry.marker
...
fighter/koopa/model/body/c28/koopadry.marker
```

A gap can truncate the detected continuous style range.

## 5. Placeholder Leakage

### Mistake

Compiling a character project that still contains template placeholders.

### Check

Search the character project for:

```text
[CHARACTER]
CHARACTER.marker
fighter_kind_[CHARACTER]
ui_chara_[CHARACTER]
```

No generic placeholders should remain in a character-specific project.

The placeholders must remain in `template/plugin-source`.

## 6. HTML-Escaped Technical Content

### Mistake

Diagnosing syntax errors merely because copied code displays:

```text
&lt;
&gt;
&amp;
&quot;
```

### Prevention

Interpret likely HTML entities before analyzing syntax:

```text
&lt;   → <
&gt;   → >
&amp;  → &
&quot; → "
```

Distinguish the serialized display from the code the user intends to execute.

## 7. Wrong GitHub Actions Branch

### Symptom

The workflow fails with:

```text
No such file or directory:
projects/dry-bowser/plugin-source
```

### Cause

The workflow was manually run from a branch that contained the workflow file but did not contain the character project path.

### Prevention

Before pressing **Run workflow**, verify the selected branch contains both:

```text
.github/workflows/<workflow>.yml
projects/<project-id>/plugin-source/
```

Do not rerun a failed job from the wrong commit. Start a new run from the correct branch.

## 8. Working Directory Mismatch

### Symptom

GitHub Actions cannot start `/usr/bin/bash` in the configured directory.

### Cause

`defaults.run.working-directory` points to a path absent from the selected branch.

### Prevention

Keep the workflow path aligned with the repository layout:

```yaml
working-directory: projects/dry-bowser/plugin-source
```

or:

```yaml
working-directory: template/plugin-source
```

## 9. Obsolete `skyline-smash` Dependency

### Symptom

Cargo cannot retrieve the old dependency source.

### Current build workaround

The workflows replace:

```text
https://github.com/blu-dev/skyline-smash.git
```

with:

```text
https://github.com/ultimate-research/skyline-smash.git
```

inside the temporary runner checkout, then remove `Cargo.lock` so Cargo resolves the maintained source.

Do not interpret the removed lockfile inside the runner as deletion from the repository.

## 10. Compilation Success Is Not Hardware Validation

### Mistake

Treating a green workflow as proof that the Echo Fighter works in-game.

### Required validation

- CSS entry appears;
- expected style count appears;
- match loads;
- model and effects load;
- returning to CSS works;
- second match works;
- results screen works;
- no crash occurs.

## 11. Artifact Size Confusion

### Mistake

Comparing the artifact ZIP size shown by GitHub with the raw `plugin.nro` size.

### Explanation

GitHub displays the compressed artifact size. The file inside can be larger.

Always inspect the extracted `plugin.nro` directly.

## 12. SHA-256 Verification

Use SHA-256 to verify:

- downloaded artifact integrity;
- equality between a new build and a tested build;
- preservation during repository migration.

PowerShell:

```powershell
$hash1 = (Get-FileHash -LiteralPath $file1 -Algorithm SHA256).Hash
$hash2 = (Get-FileHash -LiteralPath $file2 -Algorithm SHA256).Hash
$hash1 -eq $hash2
```

If the result is `True`, both files are identical bit for bit.

Dry Bowser's final tested and reorganized NRO used:

```text
9F081883E36A42DB45F4C9B815EEB70CD11EF32D2C5797E4C9D2A38627F68F54
```

## 13. GitHub Web Upload Does Not Delete Old Files

### Mistake

Uploading a reorganized tree and assuming obsolete files disappear automatically.

### Reality

GitHub web upload:

- adds new files;
- replaces files at matching paths;
- does not remove unrelated obsolete files.

After uploading a new structure, explicitly remove old root-level files and obsolete workflows.

## 14. Hidden Files May Be Missed

### Symptoms

The repository lacks:

```text
.github/
.gitignore
.gitattributes
```

### Cause

Hidden items were not included in a drag-and-drop selection.

### Prevention

Upload hidden files and `.github` explicitly, then verify their presence from the GitHub web interface.

## 15. Accidental UI Text in Names

### Mistake

Copying rendered chat text that includes interface labels such as:

```text
Mostrar más líneas
Plain Text
Copiar
```

This affected branch or commit names during the first project.

### Prevention

- copy from code blocks when possible;
- paste into a temporary plain-text editor first;
- verify branch names and commit messages before confirming;
- rename an incorrect branch immediately.

A strange commit message is cosmetically undesirable but does not alter code behavior.

## 16. Pull Request Against the Upstream Repository

### Mistake

Starting a comparison from the upstream fork parent and accidentally targeting:

```text
BigBoss320/Ultimate-Auto-Slotting
```

instead of the user's fork.

### Prevention

Verify the base repository and base branch before creating a Pull Request.

For a fork with heavily diverged history, GitHub may report:

```text
Can't automatically merge
```

When a fully validated replacement branch intentionally supersedes the old tree, changing the default branch and performing a controlled branch rename can be safer than manually resolving unrelated conflicts.

## 17. Safe Branch Replacement Procedure

The successful migration used this sequence:

```text
1. Validate the reorganized branch.
2. Compile the template and project.
3. Test the project on hardware.
4. Change the default branch to the reorganized branch.
5. Rename old main to legacy-main.
6. Rename the reorganized branch to main.
7. Compile both workflows from the new main.
8. Compare the final NRO against the tested NRO.
9. Delete legacy branches only after validation.
```

Do not delete the only known-good branch before the new default branch is validated.

## 18. PowerShell Script Execution

### Symptom

```text
execution of scripts is disabled on this system
```

### Safe one-process invocation

```bat
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "C:\Path\To\script.ps1"
```

This bypass applies only to the launched process and does not permanently change the machine policy.

### Another real failure

Using `Copy-Item -LiteralPath` with a wildcard treats the wildcard literally.

Incorrect:

```powershell
Copy-Item -LiteralPath (Join-Path $Source "*") ...
```

Correct approaches include:

```powershell
Get-ChildItem -LiteralPath $Source -Force |
    Copy-Item -Destination $Destination -Recurse -Force
```

or copying known files explicitly.

## 19. Long Code Pasted Directly Into PowerShell

### Mistake

Pasting a very long multi-line script directly into an interactive PowerShell prompt.

### Result

- partial execution;
- parser errors;
- unfinished here-strings;
- uncertain output state.

### Prevention

Provide and execute a complete `.ps1` file instead:

```bat
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "C:\Path\To\script.ps1"
```

Scripts that reorganize files should:

- validate inputs before deleting output;
- delete only the designated output directory;
- copy required files explicitly;
- verify copied hashes;
- reject forbidden binaries and archives;
- report a final inventory.

## 20. Layout DB Is Separate From Functional Registration

Dry Bowser functions without active custom layout registration, but the VS render is displaced.

Do not interpret a layout issue as proof that the character registration or gameplay redirect failed.

Treat layout changes as a separate feature and test them independently.

## 21. Final Smash Assets Are Separate

The first Dry Bowser package does not provide a slot-specific Giga Dry Bowser implementation. Regular Giga Bowser appears during Final Smash.

Do not attempt to solve this by changing unrelated global paths without testing whether the base fighter is affected.

## 22. Recommended Failure Report

When a future project fails, record:

```text
Project ID
Branch
Workflow run
Build step
Artifact hash
Base fighter
Marker path
Physical slots
Exact in-game failure point
Conflicting mods enabled
Runtime dependencies installed
Relevant logs
```

For in-game failures, specify the exact stage:

```text
Game startup
CSS load
Cursor over entry
Style change
Character confirmation
Stage selection
Match loading
Gameplay
Results
Return to CSS
Second match
```

This is more useful than saying only that the mod crashed.
