# Scripts

## GenerateExampleExpectedOutputText.ps1

Generates teext files containing the expected output for specific known test scenarios.

- Builds a list of known binary executables
  - Checks `Release` first, then falls back to `Debug`
- Runs test cases against specific executables
  - First clears the target output folder of `*.example` files (`{SolutionRoot}/{app-name}/tests/ExpectedOutput`)
  - Executes the app using specific arguments to generate a `.example` file in the above output folder
  - Substitutes some values in the output file with replaceable slugs
    - %APP_VERSION%
    - %CURRENT_YEAR%

> NOTE: There is a specific sequence that operations need to be performed in :
> - Build apps
> - Run this script
> - Run tets

## PopulateHelpText.ps1

Updates the `.md` documentation files for each app, that show the current version, example usage, and help text.

- Builds a list of known binary executables
  - Checks `Release` first, then falls back to `Debug`
- Locates and reads the documentation `.md` file for the app
- Responds to known slugs in the documentation:
  - APP_LATESTVERSION
    - Replaces the code block immediately following with the output from `{appname} -!` - showing the latest app version
  - APP_HELPOUTPUT
    - Replaces the code block immediately following with the output from `{appname} -?` - showing the latest usage help
  - APP_HELPOUTPUT {sub-command}
    - This is a variant of the above to show the help output for a specific sub-command of the app

> NOTE: This can be run any time after building the apps, but should always be run before committing.
