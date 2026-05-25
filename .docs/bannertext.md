# BannerText

## Overview

Show a text message in the console, surrounded with lines to appear as a banner.

E.g
```cmd
BannerText "Hello World"
```

```text
*******************
**  Hello World  **
*******************
```

There are options to control almost all aspects of the output

E.g. Fix the output to 80 characters

```cmd
BannerText "Hello World" -m 80
```

```text
********************************************************************************
**  Hello World                                                               **
********************************************************************************
```

And you can align the text output too

E.g. Fix the output to 80 characters

```cmd
BannerText "Hello World" -m 80 -a Center
```

```text
********************************************************************************
**                                Hello World                                 **
********************************************************************************
```

## Latest Version

[//]: # (APP_LATESTVERSION)

> bannertext -!

```text
bannertext 0.1.0-dev
```

## Full Help Text

The full help text looks something like this :

[//]: # (APP_HELPOUTPUT)

> bannertext -?

```text
bannertext v0.1.0-dev - Display a Text Banner in the console
Copyright ┬® 2025-2026 Martin Smith

Usage: bannertext [OPTIONS] <MESSAGE_TEXT>...

Arguments:
  <MESSAGE_TEXT>...  The Text to display (one or more values)

Options:
  -H, --header-line-char <HEADER_LINE_CHAR>            The character to use for header lines [default: *]
  -n, --header-line-count <HEADER_LINE_COUNT>          The number of header lines to print [default: 1]
  -F, --footer-line-char <FOOTER_LINE_CHAR>            The character to use for footer lines [default: *]
  -N, --footer-line-count <FOOTER_LINE_COUNT>          The number of footer lines to print [default: 1]
  -L, --text-line-char <TEXT_LINE_CHAR>                The character to use for text line prefix/suffix [default: *]
  -p, --title-prefix-count <TITLE_PREFIX_COUNT>        Set Title Prefix Count [default: 2]
  -s, --title-suffix-count <TITLE_SUFFIX_COUNT>        Set Title Suffix Count [default: 2]
  -P, --title-prefix-gap-size <TITLE_PREFIX_GAP_SIZE>  Set Title Prefix Gap Size [default: 2]
  -S, --title-suffix-gap-size <TITLE_SUFFIX_GAP_SIZE>  Set Title Suffix Gap Size [default: 2]
  -a, --text-alignment <TEXT_ALIGNMENT>                Set Text Alignment [default: Left] [possible values: Left, Right, Center]
  -m, --min-total-length <MIN_TOTAL_LENGTH>            Set Minimum Total line length [default: 0]
  -M, --max-total-length <MAX_TOTAL_LENGTH>            Set Maximum Total line length [default: 0]
  -h, --help                                           Print help [aliases: -?]
  -V, --version                                        Print version [aliases: -!]

Examples:
  bannertext "Hello World"
  bannertext "Hello World" --min-total-length 80
  bannertext "Hello World" --min-total-length 80 --text-alignment Center
  bannertext "Hello World" --header-line-char '#' --footer-line-char '#' --text-line-char '#'
```
