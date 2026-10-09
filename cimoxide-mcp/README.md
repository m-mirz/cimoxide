# cimoxide-mcp

`cimmcp` is a [Model Context Protocol](https://modelcontextprotocol.io) server for ENTSO-E
**CGMES** and **Network Code** RDF/XML, built on
[cimoxide](https://github.com/m-mirz/cimoxide). It lets an LLM chat answer questions about
a model: which generators a model has and their set points, why an object fails
validation, what a CIM class or attribute means.

Its tools are read-only:

| Tool | |
|---|---|
| `list_model_sets` | directories holding CIM files, with each file's profiles |
| `summary` | a model set's files, model headers and object counts by class |
| `find_objects` | search by class (abstract classes included), name/mRID text and attribute value |
| `get_object` | one object's fields, references resolved, and what references it |
| `validate` | the ENTSO-E SHACL and SPARQL rules, as `cimcli validate` runs them |
| `sparql` | a SPARQL query over the merged CGMES data |
| `describe_class` | definitions of classes, attributes and enumerations from the ENTSO-E vocabulary |

A model set is every CIM XML file in one directory. `--dir <path>` (else
`CIMOXIDE_MODEL_DIR`, else the working directory) is where relative paths resolve.

## Install

With [uv](https://docs.astral.sh/uv/), nothing needs installing first; clients start it as

```
uvx --from cimoxide-mcp cimmcp --dir /path/to/models
```

or install it once with `uv tool install cimoxide-mcp` (or `pipx install cimoxide-mcp`) and
run `cimmcp`. It is also on crates.io (`cargo install cimoxide-mcp`), as binaries on each
[GitHub release](https://github.com/m-mirz/cimoxide/releases), and bundled in the
[VS Code extension](https://marketplace.visualstudio.com/items?itemName=m-mirz.cimoxide).

## Configure a client

Claude Code (servers start in the project directory, so `--dir` can be left out):

```
claude mcp add cimoxide -- uvx --from cimoxide-mcp cimmcp
```

Codex (`~/.codex/config.toml`):

```toml
[mcp_servers.cimoxide]
command = "uvx"
args = ["--from", "cimoxide-mcp", "cimmcp", "--dir", "/path/to/models"]
```

Claude Desktop, Cursor, Windsurf, Antigravity and most others take an `mcpServers` block:

```json
{
  "mcpServers": {
    "cimoxide": {
      "command": "uvx",
      "args": ["--from", "cimoxide-mcp", "cimmcp", "--dir", "/path/to/models"]
    }
  }
}
```

RDFS and SHACL can be loaded at runtime instead of the built-in tables with
`CIMOXIDE_RDFS_DIR` / `CIMOXIDE_SHACL_DIR` in the server's environment.
