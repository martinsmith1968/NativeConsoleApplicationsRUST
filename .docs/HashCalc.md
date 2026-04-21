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

> BannerText -!

```text
v2.2.0.0-dev
```

## Full Help Text

The full help text looks something like this :

[//]: # (APP_HELPOUTPUT)

> BannerText -?

```text
BannerText v2.2.0.0-dev - Display a Text Banner in the console
Copyright ® 2018-2026 Martin Smith

Usage:
BannerText [message-text] [OPTIONS]

Options:
[message-text] Text                     The Text to display (Required)
-hlc, --header-line-char Char           The character to use for header lines (Default:*)
-hln, --header-line-count Integer       The number of header lines to print (Default:1)
-flc, --footer-line-char Char           The character to use for footer lines (Default:*)
-fln, --footer-line-count Integer       The number of footer lines to print (Default:1)
-tlc, --text-line-char Char             The character to use for text line prefix/suffix (Default:*)
-tpc, --title-prefix-count Integer      Set Title Prefix Count (Default:2)
-tsc, --title-suffix-count Integer      Set Title Suffix Count (Default:2)
-tpgs, --title-prefix-gap-size Integer  Set Title Prefix Gap Size (Default:2)
-tsgs, --title-suffix-gap-size Integer  Set Title Suffix Gap Size (Default:2)
-ta, --text-alignment Value             Set Text Alignment (Default:Left, Values: Left, Right, Center)
-minl, --min-total-length Integer       Set Minimum Total line length (Default:0)
-maxl, --max-total-length Integer       Set Maximum Total line length (Default:0)
-!, --version +/-                       Show App Version details (Default:false)
-?, --help +/-                          Show Help screen (Default:false)
-@, --use-default-arguments-file +/-    Use Default Arguments File (BannerText.options) (Default:true)
-$, --use-local-arguments-file +/-      Use Local Arguments File (BannerText.options) (Default:true)

Default App arguments can be specified in : C:\Temp\a6e4aa6c3091438b97abf6d2b52171ed\BannerText.options
Local App arguments can be specified in : C:\Temp\BannerText.options
```
