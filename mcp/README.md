# MCP Layer — rust-forge-skill Agent Integration

This directory provides a machine-readable manifest and integration instructions that allow AI agents and tool runners to discover, load, and use `rust-forge-skill` programmatically — without manual copying or path assumptions.

---

## Architecture

```
rust-forge-skill/
├── mcp/
│   ├── README.md                    ← This file
│   ├── manifest.example.json       ← Machine-readable skill manifest
│   ├── tool-contract.md            ← Agent protocol contract
│   └── agent-loader-instructions.md ← Step-by-step agent instructions
├── SKILL.md                        ← Primary agent entrypoint (read first)
└── [guides, templates, scripts, checklists, ci, examples]
```

The MCP layer does **not** replace `SKILL.md`. It augments it with:
1. A machine-readable **manifest** for auto-discovery
2. A formal **tool contract** for agent protocol compliance
3. **Loader instructions** for agents that need step-by-step guidance

---

## Loading Methods

### 1. Claude Code (via Skill tool)

Claude Code's built-in `Skill` tool loads `SKILL.md` as the entrypoint. No manifest needed — the agent reads `SKILL.md` at session start automatically.

To explicitly invoke:
```
/skill rust-forge-skill   # or the path to SKILL.md
```

The `mcp/manifest.example.json` exists so that **wrapper tools** (skill registries, agent launchers) can parse it and generate skill metadata.

---

### 2. IDE Agents (VS Code Copilot, JetBrains, etc.)

IDE agents that support custom skill packs can load this pack by:

1. Setting a `SKILL_PATH` env var or plugin config pointing to the `rust-forge-skill/` directory
2. The agent reads `SKILL.md` as the operating contract
3. The manifest (`mcp/manifest.example.json`) provides metadata for the IDE's skill registry UI (name, description, version, capabilities)

**VS Code Copilot extension authors:** Use `manifest.example.json` as a reference to generate a `copilot-skills.json` entry:

```json
{
  "name": "rust-forge-skill",
  "version": "1.0",
  "description": "Production-grade Rust project scaffolding and validation",
  "entrypoint": "SKILL.md",
  "capabilities": ["rust_project_scaffold", "rust_project_audit", ...]
}
```

---

### 3. Local CLI Agents (CLI tools with skill loading)

Any CLI agent that supports a `--skill` flag or config entry can use this pack:

```bash
# Example: my-agent CLI with skill loading
my-agent --skill ./rust-forge-skill --task "scaffold a new CLI project called mytool"

# Or via config (~/.config/my-agent.yaml):
# skills:
#   - name: rust-forge-skill
#     path: /path/to/rust-forge-skill
#     enabled: true
```

The agent loader instructions (`mcp/agent-loader-instructions.md`) provide the exact sequence the agent must follow.

---

### 4. MCP-Compatible Tool Runners

Model Context Protocol (MCP) servers that host skill packs can use `manifest.example.json` to advertise this skill:

```bash
# MCP server config example (mcp_server_config.json)
{
  "skills": [
    {
      "name": "rust-forge-skill",
      "manifest": "/path/to/rust-forge-skill/mcp/manifest.example.json",
      "root": "/path/to/rust-forge-skill"
    }
  ]
}
```

The tool contract (`mcp/tool-contract.md`) defines the exact input/output contract the MCP server must enforce when routing tasks to this skill.

---

### 5. Repo-Scanning Agents (automated discovery)

Agents that scan repositories for available skill packs (e.g., in a CI pre-flight check or a multi-repo management tool) can:

1. Look for a `mcp/` directory at the repo root
2. Parse `mcp/manifest.example.json` to get skill metadata
3. Verify the manifest version is compatible with the agent's protocol version
4. Load the skill if the agent's task matches one of the declared capabilities

```bash
# Example: repo-scan agent discovery
ls ./*/mcp/manifest.example.json   # scan all repos for MCP skills
jq -r '.name + " (" + .version + ")"' rust-forge-skill/mcp/manifest.example.json
# → rust-forge-skill (1.0)
```

---

## Manifest

`mcp/manifest.example.json` is the **authoritative machine-readable description** of this skill. It contains:

- `name`, `version`, `description`
- `entrypoint` (always `SKILL.md`)
- Enumerated lists of guides, templates, scripts
- `capabilities` — the task types this skill can handle
- `validation_commands` — the quality gates an agent must run
- `expected_outputs` — what a conforming delivery looks like

Rename to `manifest.json` for use with tools that require a live manifest file.

---

## Tool Contract

`mcp/tool-contract.md` defines the **formal protocol** between a calling agent and this skill. It specifies:

- **Input schema** — required fields an agent must provide
- **Output schema** — what the agent must produce
- **Safety invariants** — hard rules the agent must not break
- **Phase descriptions** — inspect, plan, scaffold/refactor, validate, report

Agents that implement the tool contract are guaranteed to produce conforming output from this skill.

---

## Loading Checklist

Before using this skill in an agent, verify:

- [ ] `SKILL.md` is readable at the skill root
- [ ] `guides/` directory contains all referenced guides
- [ ] `templates/` directory contains at least one template
- [ ] `scripts/` contains `validate_rust_project.sh`, `audit_unsafe.sh`, `check_msrv.sh`, `generate_cargo_workspace.sh`
- [ ] Agent has read permissions on the entire skill directory
- [ ] Agent has write permissions in the target project directory (if scaffolding)
- [ ] Agent has execute permissions on shell scripts in `scripts/`

---

## Versioning

The manifest uses semver. The skill version and manifest version are kept in sync:

| Skill Version | Manifest Version | Notes |
|---|---|---|
| 1.0.x | 1.0 | Initial release |

When updating the skill:
1. Bump `version` in `manifest.example.json`
2. Add a changelog entry to this file
3. Update `SKILL.md` version comment

---

## Support

- Issues: https://github.com/your-org/rust-forge-skill/issues
- The skill is MIT OR Apache-2.0 licensed
