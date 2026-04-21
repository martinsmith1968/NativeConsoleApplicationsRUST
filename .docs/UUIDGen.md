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

> PauseN -!

```text
v2.2.0.0-dev
```

## Full Help Text

The full help text looks something like this :

[//]: # (APP_HELPOUTPUT)

> PauseN -?

```text
PauseN v2.2.0.0-dev - Pause console output for a keypress, or a timeout
Copyright ® 2018-2026 Martin Smith

Usage:
PauseN [message-text] [OPTIONS]

Options:
[message-text] Text                   The Text to display (Default:Press any key to continue (or wait {timeout} seconds) . . . )
-t, --timeout Integer                 The timeout to wait for in seconds (Default:30)
-s, --sleep Integer                   The period to sleep for between checks for in milliseconds (Default:200)
-e, --escape-cancels-timeout +/-      Allow ESC to cancel timeout (Default:true)
-c, --timeout-cancelled-text Text     The text to show when a timeout is cancelled (Default:(Timeout Cancelled))
-!, --version +/-                     Show App Version details (Default:false)
-?, --help +/-                        Show Help screen (Default:false)
-@, --use-default-arguments-file +/-  Use Default Arguments File (PauseN.options) (Default:true)
-$, --use-local-arguments-file +/-    Use Local Arguments File (PauseN.options) (Default:true)

Default App arguments can be specified in : C:\Temp\a6e4aa6c3091438b97abf6d2b52171ed\PauseN.options
Local App arguments can be specified in : C:\Temp\PauseN.options
```
