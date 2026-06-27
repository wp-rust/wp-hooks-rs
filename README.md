# wp-hooks-rs

Rust port of [`@wordpress/hooks`](https://github.com/WordPress/gutenberg/tree/trunk/packages/hooks) from the Gutenberg project.

Provides `add_filter`, `add_action`, `apply_filters`, and `do_action` — the WordPress plugin hook system for building extensible Rust tools. Lets users customize behavior without forking your code.

## Installation

```toml
[dependencies]
wp-hooks-rs = "0.1"
```

## Usage

```rust
use wp_hooks_rs::Hooks;

let mut hooks = Hooks::new();

// Register a filter that modifies block HTML output
hooks.add_filter(
    "my_tool/heading_html",  // hook name
    "my-plugin",             // namespace
    10,                      // priority (lower runs first)
    |html: String| html.replace("<h2>", "<h2 class=\"custom\">"),
);

// Apply all registered filters
let result = hooks.apply_filters(
    "my_tool/heading_html",
    "<h2>Hello</h2>".to_string(),
);
assert!(result.contains("class=\"custom\""));

// Register an action (no return value)
hooks.add_action("my_tool/post_convert", "my-plugin", 10, |post_id: u64| {
    println!("Converted post {}", post_id);
});

hooks.do_action("my_tool/post_convert", 42u64);
```

## API

| Function | WordPress Equivalent | Description |
|---|---|---|
| `add_filter(hook, ns, priority, fn)` | `add_filter()` | Register a filter callback |
| `add_action(hook, ns, priority, fn)` | `add_action()` | Register an action callback |
| `apply_filters(hook, value)` | `apply_filters()` | Run all filters, return modified value |
| `do_action(hook, value)` | `do_action()` | Run all actions (no return value) |
| `remove_filter(hook, ns)` | `remove_filter()` | Remove all filters for a namespace |
| `remove_action(hook, ns)` | `remove_action()` | Remove all actions for a namespace |

## Use Cases

- Make your CLI tool's block output customizable without forking
- Allow third-party code to intercept and modify conversion logic
- Port WordPress plugin patterns to server-side Rust tooling

## Related Crates

| Crate | Purpose |
|---|---|
| [`wp-block-parser-rs`](https://crates.io/crates/wp-block-parser-rs) | Parse Gutenberg block markup |
| [`wp-style-engine-rs`](https://crates.io/crates/wp-style-engine-rs) | Compile block style objects to CSS |

## License

GPL-2.0-or-later — consistent with the [Gutenberg project](https://github.com/WordPress/gutenberg).
