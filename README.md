# Zed PyLSP extension

This extension add supporting to the PyLSP to Zed.

## Installation

Go to Zed's extension manager and search for vale. Click install.

## Configuration

Install the pylsp inside your virtual env:

Basic Support:

- `pip install python-lsp-server`

With All Dependencies:

- `pip install "python-lsp-server[all]"`

This will expose the command pylsp on your PATH. Confirm that installation succeeded by running pylsp --help.

If the respective dependencies are found, the following optional providers will be enabled:

```
  Rope for Completions and renaming
  Pyflakes linter to detect various errors
  McCabe linter for complexity checking
  pycodestyle linter for style checking
  pydocstyle linter for docstring style checking (disabled by default)
  autopep8 for code formatting
  YAPF for code formatting (preferred over autopep8)
  flake8 for error checking (disabled by default)
  pylint for code linting (disabled by default)
```

## Add as a language server
```
{
  "languages": {
    "Python": {
      "language_servers": ["pyright", "pylsp"]
    }
  }
}
```

## Contributing

I ❤️ Pull Requests, but I do like well documented summaries and commit messages.

Every new Pull Request will notify me, I will review it and merge it if it looks good.

License Apache 2.0
