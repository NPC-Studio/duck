# duck

![GitHub branch checks state](https://img.shields.io/github/checks-status/imlazyeye/duck/main)
![GitHub](https://img.shields.io/github/license/imlazyeye/duck)

A collection of customizable lints to identify common mistakes in GML ([GameMaker Language](https://manual.yoyogames.com/#t=Content.html)).

Currently supports [33 lints](LINTS.md), with more on the way!

`duck` is is a highly opinionated linter that enables far stricter rules for GML than GameMaker itself enforces. It is able to detect code that will directly lead to errors as well as enforce styling rules -- all of which are **completely customizable**.

## Table of Contents

- [Customization](#customization)
  - [Lint Levels](#lint-levels)
  - [Lint Options](#lint-options)
  - [Tags](#tags)
- [Examples](#examples)
- [Usage Guide](#usage-guide)
  - [Instalation](#instalation)
  - [Creating a configuration file](#creating-a-configuration-file)
  - [Setting lint levels](#setting-lint-levels)
  - [Running the linter](#running-the-linter)
- [Contributing](#contributing)
- [Support and Requests](#support-and-requests)

## Customization

While `duck` expresses strong opinons on the GML it reads, those opinons are enitrely in your control.

### Lint Levels

`duck` can use a configuration file per-project to change how it behaves. The most basic adjustment you can make is overriding the default "level" of any lint.

```toml
[lint-levels]
and_keyword = "allow"
try_catch = "warn"
constructor_without_new = "deny"
```

This demonstrates the three different levels: "allow" will tell `duck` to fully ignore the lint, "warn" will mark them as warnings, and "deny" will treat them like full errors.

### Lint options

Some lints come with customizable behavior. `english_flavor_violation`, for example, let's you decide between the British or American spelling of GML functions. `var_prefix_violation` let's you decide if you think local variables should be prefixed with an underscore (`_foo`) or with nothing at all (`foo`).

```toml
english_flavor = "american"
var_prefixes = false
```

### Tags

Sometimes you need to break the rules. Perhaps there is a place in my codebase that I would really like to use a `globalvar` even though it is depreacted. In general though, I still don't want them to be allowed. You can tag the specific occurance of the issue to acknowledge (and ignore) the lint.

```gml
// #[allow(deprecated)]
globalvar my_globalvar;
```

Tags are a great way to enable lints on things you don't want to _fully_ ban, but want to keep a close eye on.

## Examples

Let's use one of `duck`'s more powerful lints as an example: `missing_case_member`.

```gml
emum MyEnum {
    A,
    B,
    C,
}

switch my_enum {
    case MyEnum.A: break;
    case MyEnum.B: break;
}
```

While this code is acceptable GML, it contains a danger: we do not have a `case` set up if `my_enum` is equal to `MyEnum.C`. Perhaps we did not consider, `MyEnum.C` when writing this code, or maybe it was implemented after this code was written.

Normally, this kind of an issue is difficult to detect. With `duck`, it's trivial:

\<image>

As the suggestions there mention, there's a few ways we could resolve this. We could, of course, add in a case for `MyEnum.C`. We could also add a `default` case to our switch -- `duck` would then recognize that all the bases are covered. We could customize that behavior further by telling `duck` to ignore this error if we have a default case, _unless_ that default cases requests the game to crash -- then `duck` will recognize that the default case is not an intended outcome.

## Usage

To install, you have two options:

### Instalation

To install manually, do the following:

1. Download the latest release here
2. Add `duck` to your PATH environment variable (optional)
   - You can pass `duck` a path directly when using it, but adding it to your `PATH` will be much more convenient

If you're a Rust developer, you can just run `cargo install --git https://github.com/imlazyeye/duck` .

### Creating a configuration file

It is highly recommended to use a configuration file with `duck`. To do so, navigate in your terminal to the directory of your project and run `duck new-config`.

This will create a file called `.duck.toml` in your project's directory that will be used on subsequent runs of `duck`. Opening this file will reveal many pre-set properties, some of which you may be able to able to adjust without any instruction. Either way, a full list of the possible values your config can hold are below.

### Configuration options

| Property                 | Possible Values       | Explanation                                                                                                                                       |
| ------------------------ | --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| todo_keyword             | Any string            | The name of a function in your code base used to mark something as unfinished. Used by `todo`.                                                    |
| max_arguments            | Any number            | The maximum number of arguments the associated lint will allow. Used by `too_many_arguments`.                                                     |
| statement_parentheticals | true, false           | Whether or not statements should have parenthesis over their condition (ex: `if (foo)` vs `if foo`). Used by `statement_parenthetical_violation`. |
| var_prefixes             | true, false           | Whether or not local variables should be prefixed with an underscore (ex: `var _foo` vs `var foo`). Used by `var_prefix_violation`.               |
| english_flavor           | "american", "british" | The spelling of English words you prefer for GameMaker functions (ex: `color` vs `colour`). Used by `english_flavor_violation`.                   |
| length_enum_member_name  | Any string            | A name to ignore in enums that denote its length (ie: `Len`, `Count`). Used by `missing_case_member`.                                             |

### Setting lint levels

You can additionally section called `[lint_levels]` to specify global lint levels for specific lints. You can see a working example of this [here](#lint-levels).

### Running the linter

To run the lint, simply run the lint command!

```
duck lint
```

If you would like to run the linter on a project outside the current directory you are in, you can pass a path like so:

```
duck lint --path path/to/project
```

## Contributing

`duck` is designed to be easily extensible, and contributions are extremely welcome! Please see [Contributing](CONTRIBUTING.md) for more information.

## Support and Requests

Please [open an issue](https://github.com/imlazyeye/duck/issues) if you encounter any problems with `duck`, or if you have any feature requests you would like to make!