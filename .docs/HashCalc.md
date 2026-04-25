# HashCalc

## Overview

Calculate the hash of some text or a file contents, via a variety of algorithms.

E.g
```cmd
hashcalc -t "Hello World"
```

```text
Hello World [sha256] : a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e
```

We can control the algorithm used:

```cmd
hashcalc -t "Hello World" -a sha512
```

```text
Hello World [sha512] : 2c74fd17edafd80e8447b0d46741ee243b7eb74dd2149a0ab1b9246fb30382f27e853d8585719e0e67cbda0daa8f51671064615d645ae27acb15bfb1447f459b
```

We can also calculate the hash of files too:

```cmd
hashcalc -f "LICENSE"
```

```text
LICENSE [sha256] : 92d2c19c6409ab0401dea84dac4edd4b5b253975a3f1e1d39b6d6faf31b381ee
```

And optionally write the result to a file :

```cmd
hashcalc -f "LICENSE" --write
```

```text
cat LICENSE.sha256
LICENSE [sha256] : 92d2c19c6409ab0401dea84dac4edd4b5b253975a3f1e1d39b6d6faf31b381ee
```

## Latest Version

[//]: # (APP_LATESTVERSION)

> hashcalc -!

```text
hashcalc 0.1.0-dev
```

## Full Help Text

The full help text looks something like this :

[//]: # (APP_HELPOUTPUT)

> hashcalc -?

```text
hashcalc v0.1.0-dev - Generate a hash of text or file contents
Copyright ┬® 2025-2026 Martin Smith

Usage: hashcalc.exe [OPTIONS]

Options:
  -t, --text <TEXT>            The text to generate a hash for (mutually exclusive with --file)
  -f, --file <FILE>            Path to file to hash
  -w, --write                  Write output to file instead of stdout (requires --file)
  -a, --algorithm <ALGORITHM>  Hash algorithm to use: sha1, md5, sha256, sha512, base64 [default: sha256]
  -h, --help                   Print help [aliases: -?]
  -V, --version                Print version [aliases: -!]

Examples:
  hashcalc --text "Hello World"
  hashcalc --text "Hello World" --algorithm md5
  hashcalc --file myfile.txt
  hashcalc --file myfile.txt --algorithm sha512
  hashcalc --file myfile.txt --write
```
