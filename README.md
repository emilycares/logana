# logana

A build log analysis tool so that your PDE (Personal Development Environment) can have a list of files that contain errors.
Currently, it only supports

- Maven
- Gradle (java only) (Tests are not supported for the moment)
- KarmaJasmine
- Cargo
- java (Only exceptions)
- [https://github.com/crate-ci/typos](https://github.com/crate-ci/typos)
- zig
- eslint
with plans for more.

## Compatibility

Regularly tested on

- linux
- Windows 10

## Installation

Install the rust.

```bash
git clone https://github.com/micmine/logana
cd logana

cargo build --release
```

And add it to your path

## Usage

### command

In this case the parser will be guessed by the command.

```bash
logana -c "cargo build --color always"
```

### stdin

In bash "|&" will also pipe sterr.

```bash
mvn clean install |& logana --parser maven --input stdin
```

It will create an output file named ".logana-report".

### How i use logana

I use logana to get a faster feedback loop from a build error to a mistake I made in some file.
For working on rust projects, i use:

```bash
find **/*.rs | entr logana -c "cargo test --color always"
```
or the new way
```bash
logana -w -c "cargo test --color always"
```
Before I have used the same thing just without "logana -c". The difference is to before that I can instantly see all errors in my editor without going throw all the log. In most languages and tools, this is pretty straight forward. But you still need to search for the file and go to the right place. With the generated .logana-report i can press one shortcut in my editor to get a list of these errors and can jump to them.
I started to creating this tool to simplify this process when I am working on my work angular project. There the test results are not very friendly to read and sometimes. You just miss failing test cases if you don't look correctly. That is why I started to creating this tool. And I have found this tool so useful that started creating other parsers for different languages to keep this workflow.

## Editor support

The editor support allows the editor to parse the ".logana-report" into its now error list.
Plugins are available here:

- neovim [https://github.com/micmine/logana.nvim](https://github.com/micmine/logana.nvim)
- jetbrains [https://github.com/micmine/logana-jetbrains](https://github.com/micmine/logana-jetbrains)
