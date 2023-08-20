# logana

A build log analysis tool so that your PDE (Personal Development Environment) can have a list of files that contain errors.
Currently, it only supports

- Cargo
- Gradle (java only) (Tests are not supported for the moment)
- KarmaJasmine
- Maven
- dune
- eslint
- go
- java (Only exceptions)
- typos https://github.com/crate-ci/typos
- zig

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
logana -w -c "cargo test"
```
or the other way
```bash
find **/*.rs | entr logana -c "cargo test --color always"
```

### Why do i want this?
The point is there are times where this does no hold up or is inconsistant. I have started to build this tool because i sometimes i get different errors in the compiler. Also i want to jump easily to printed paths to sourcecode. Regardless if it is in compilation unit test or at runntime. That is the goal.

## Editor support

The editor support allows the editor to parse the ".logana-report" into its now error list.
Plugins are available here:

- neovim [https://github.com/micmine/logana.nvim](https://github.com/micmine/logana.nvim)
- jetbrains [https://github.com/micmine/logana-jetbrains](https://github.com/micmine/logana-jetbrains)
