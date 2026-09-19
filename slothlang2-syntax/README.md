# SlothLang 2 Syntax Highlighting

VS Code syntax highlighting and editing support for the SlothLang 2
programming language.

## Features

- Syntax highlighting for `.sl` source files and `.slt` module/stdlib files.
- Keywords and declarations: `func`, `class`, `trait`, `impl`, `pub`, `var`,
  `let`, `if`/`else`, `while`, `for`, `return`, `break`, `continue`,
  `import`/`as`, `extern`, `and`/`or`/`not`/`is`.
- Contextual type keywords: `int`, `float`, `bool`, `str`, `unit`, `range`,
  `any`, `dyn`, `Array`, `Map`, `Weak`, `Tensor`, `Fiber`, and more.
- Literals: integers, floats (including scientific notation), booleans,
  `nil`, `this`/`super`.
- Double-quoted strings with escape sequences and `${expr}` interpolation
  (interpolated expressions are highlighted recursively).
- Comments: `//` line comments and `/* ... */` block comments.
- Operators including `|>`, `?:`, `..`, `..=`, `...`, `->`, `&&`, `||`,
  compound assignment and bitwise operators.
- Language configuration: bracket matching, auto-closing pairs, comment
  toggling and indentation rules.

## Supported file types

| Extension | Use |
| --- | --- |
| `.sl` | SlothLang source programs |
| `.slt` | SlothLang modules / standard library files |

## Usage

Install the extension (or press `F5` in this folder to launch an Extension
Development Host), then open any `.sl` / `.slt` file. Highlighting is applied
automatically.

## Development

- `package.json` registers the language and grammar contributions.
- `syntaxes/slothlang2.tmLanguage.json` is the TextMate grammar.
- `language-configuration.json` defines comments, brackets and indentation.

To test, open this folder in VS Code and run the `Extension` launch
configuration (`F5`). Use `Developer: Inspect Editor Tokens and Scopes` to
debug token scopes.

## Release Notes

See [CHANGELOG.md](./CHANGELOG.md).
