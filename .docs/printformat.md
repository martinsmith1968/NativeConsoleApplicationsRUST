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

## C# Cultures

With `--format-type CSharp`, the `N`, `C`, `F`, `E` and `G` format specifiers use the
separators and currency layout of a culture. `--culture` takes one of the built-in
names (`en-GB` (the default), `en-US`, `de-DE`, `fr-FR`; case-insensitive) or the path
to a culture file.

```cmd
PrintFormat -t CSharp --culture de-DE "{0:N2} | {1:C}" 1234567.891 1234.5
```

```text
1.234.567,89 | 1.234,50 €
```

### Culture Files

A culture file is a TOML file. Every field is optional; missing values come from `base`
(or `en-GB` if there is no `base`).

```toml
# swiss.toml
base = "de-DE"                      # built-in culture to start from
decimal_separator = "."
group_separator = "’"
currency_symbol = "CHF"
currency_position = "before-space"  # before | after | before-space | after-space
```

```cmd
PrintFormat -t CSharp --culture swiss.toml "{0:C}" 1234.5
```

```text
CHF 1’234.50
```

Built-in names are checked first, so to use a file with the same name as a built-in
culture, prefix it with a path (e.g. `./en-GB`). `--currency`, `--decimal-separator` and
`--group-separator` override the culture's values. Arguments are always read with `.`
as the decimal point.

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
Copyright © 2025-2026 Martin Smith

Usage: printformat [OPTIONS] <FORMAT_STRING> [ARGUMENTS]...

Arguments:
  <FORMAT_STRING>  The format string (use {} as placeholders)
  [ARGUMENTS]...   Arguments to substitute into the format string

Options:
  -t, --format-type <FORMAT_TYPE>              Format type to use for the format string [default: Rust] [possible values: Rust, CSharp, C]
      --culture <CULTURE>                      Culture whose separators and currency the C# format type uses: en-GB, en-US, de-DE, fr-FR, or the path to a culture file [default: en-GB]
      --currency <CURRENCY>                    Currency symbol for the C# 'C' format specifier (overrides the culture's)
      --decimal-separator <DECIMAL_SEPARATOR>  Decimal separator for C# number formats (overrides the culture's)
      --group-separator <GROUP_SEPARATOR>      Group (thousands) separator for the C# 'N' and 'C' format specifiers (overrides the culture's)
  -h, --help                                   Print help [aliases: -?]
  -V, --version                                Print version [aliases: -!]

Examples:
  printformat "Hello, {}!" "World"
  printformat "{} + {} = {}" "1" "2" "3"
  printformat "{:>10}" "right"
  printformat "{:<10}" "left"
  printformat "{:*^20}" "center"
  printformat "No placeholders"
  printformat --format-type CSharp "{0} is {1} years old" "Alice" "30"
  printformat --format-type CSharp "{0,-10} | {1,10}" "left" "right"
  printformat --format-type CSharp "{0:D5}" "42"
  printformat --format-type CSharp "{0:N2}" "1234567.891"
  printformat --format-type CSharp "{0:X4}" "255"
  printformat --format-type CSharp --currency "$" "{0:C}" "1234.5"
  printformat --format-type CSharp --culture de-DE "{0:N2} | {1:C}" "1234567.891" "1234.5"
  printformat --format-type C "%s is %d years old" "Alice" "30"
  printformat --format-type C "%10s | %-10s" "right" "left"
  printformat --format-type C "%05d" "42"
  printformat --format-type C "%.2f" "3.14159"

See Also:
 https://docs.rs/strfmt/latest/strfmt/
 https://doc.rust-lang.org/std/fmt/
 https://cplusplus.com/reference/cstdio/printf/
```
