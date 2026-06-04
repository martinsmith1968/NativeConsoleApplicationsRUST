# PrintFormat

## Overview

Show a formatted text message in the console, using parameters as input to built-in string formatting library.

E.g
```cmd
PrintFormat "The answer to life, the Universe and everything is {}" 42
```

```text
The answer to life, the Universe and everything is 42
```

There are options to control formatting of individual fields

E.g. Pad string to specific length

```cmd
PrintFormat "Name: {:<20}, Age {:0}" Bob 42
```

```text
Name: Bob                 , Age 42
```

## Latest Version

[//]: # (APP_LATESTVERSION)

> printformat -!

```text
printformat 0.1.0-dev
```

## Full Help Text

The full help text looks something like this :

[//]: # (APP_HELPOUTPUT)

> printformat -?

```text
printformat v0.1.0-dev - Format and print text using a format string and arguments
Copyright ┬® 2025-2026 Martin Smith

Usage: printformat <FORMAT_STRING> [ARGUMENTS]...

Arguments:
  <FORMAT_STRING>  The format string (use {} as placeholders)
  [ARGUMENTS]...   Arguments to substitute into the format string

Options:
  -h, --help     Print help [aliases: -?]
  -V, --version  Print version [aliases: -!]

Examples:
  printformat "Hello, {}!" "World"
  printformat "{} + {} = {}" "1" "2" "3"
  printformat "{:>10}" "right"
  printformat "{:<10}" "left"
  printformat "{:*^20}" "center"
  printformat "No placeholders"
```
