# UUIDGen

## Overview

This tool offers a simple, convenient built in way of generating Unique IDs from the command line. It supports `GUID` and `NanoId` algorithms

E.g. For GUIDs

```cmd
uuidgen 
```

```text
6e235adc-94d1-441e-a6db-69bab4ca9fb7
```

E.g. For NanoIds

```cmd
uuidgen -t nanoid
```

```text
lRz_pdvYHNV0NIemvXabl
```

And we can generate multiple all at once :

```cmd
uuidgen -c 5
```

```text
25bc61bf-0d29-4749-92b0-31dbc91737f2
18be3210-bb25-4234-99c2-be9004263b35
09d9222e-5c87-4222-a419-9796b025036c
cdcbb15f-7faf-4a49-bd45-afe613c07ced
7462ecf2-9314-4721-856f-a505e86bee27
```

And control the output formatting :

```cmd
uuidgen -u -y
```

```text
E3C791C0E66D4D418164F6825CE8DE84
```

## Latest Version

[//]: # (APP_LATESTVERSION)

> uuidgen -!

```text
uuidgen 0.1.0-dev
```

## Full Help Text

The full help text looks something like this :

[//]: # (APP_HELPOUTPUT)

> uuidgen -?

```text
uuidgen v0.1.0-dev - Generate Unique IDs (UUIDs) with controlled output and formatting
Copyright ┬® 2025-2026 Martin Smith

Usage: uuidgen.exe [OPTIONS]

Options:
  -c, --count <COUNT>                      Number of times to generate [default: 1]
  -t, --uuid-type <UUID_TYPE>              The type of UUID to generate [default: guid] [possible values: guid, nanoid]
  -v, --guid-version <GUID_VERSION>        The version of GUID to generate [default: v4] [possible values: v4, v6, v7]
  -l, --nanoid-length <NANOID_LENGTH>      The size of NanoId to generate [default: 21]
  -y, --non-hyphenated                     Format the GUID without Hyphens (GUID only)
  -u, --uppercase                          Covert the GUID to Upper case values (GUID only)
  -o, --output-template <OUTPUT_TEMPLATE>  The template to use when writing the output [default: {uuid}]
  -6, --guid-v6-seed <GUID_V6_SEED>        The seed to use when generating a v6 Guid (6 values) [default: 1,2,3,4,5,6]
  -h, --help                               Print help [aliases: -?]
  -V, --version                            Print version [aliases: -!]

NOTE:
output-template supports: {uuid}, {sequence} dynamic values
(See also : https://github.com/vitiral/strfmt)

Examples:
  uuidgen
  uuidgen --count 5
  uuidgen --count 3 --uppercase
  uuidgen --uuid-type nanoid
  uuidgen --count 5 --output-template "{sequence}: {uuid}"
```
