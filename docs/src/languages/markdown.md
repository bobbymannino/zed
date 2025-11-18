# Markdown

Markdown support is available natively in Zed.

- Tree-sitter: [tree-sitter-markdown](https://github.com/tree-sitter-grammars/tree-sitter-markdown)
- Language Server: N/A

## Syntax Highlighting Code Blocks

Zed supports language-specific syntax highlighting of markdown code blocks by leveraging [tree-sitter language grammars](../extensions/languages.md#grammar). All [Zed supported languages](../languages.md), including those provided by official or community extensions, are available for use in markdown code blocks. All you need to do is provide a language name after the opening <kbd>```</kbd> code fence like so:

````python
```python
import functools as ft

@ft.lru_cache(maxsize=500)
def fib(n):
    return n if n < 2 else fib(n - 1) + fib(n - 2)
```
````

## Configuration

### Markdown Preview

Zed provides a built-in markdown preview that can be opened using {#action markdown::OpenPreviewToTheSide}. You can also configure markdown files to automatically open a preview when the file is opened:

```json [settings]
  "markdown_preview": {
    "auto_open_preview": true
  }
```

When enabled, opening any markdown file will automatically open a preview pane to the side, allowing you to see your rendered markdown as you edit.

### Format

Zed supports using Prettier to automatically re-format Markdown documents. You can trigger this manually via the {#action editor::Format} action or via the {#kb editor::Format} keyboard shortcut. Alternately, you can automatically format by enabling [`format_on_save`](../configuring-zed.md#format-on-save) in your settings.json:

```json [settings]
  "languages": {
    "Markdown": {
      "format_on_save": "on"
    }
  },
```

### Trailing Whitespace

By default Zed will remove trailing whitespace on save. If you rely on invisible trailing whitespace being converted to `<br />` in Markdown files you can disable this behavior with:

```json [settings]
  "languages": {
    "Markdown": {
      "remove_trailing_whitespace_on_save": false
    }
  },
```
