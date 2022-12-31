# logana

A build log analysis tool so that your PDE (Personal Development Environment) can have a list of files that contain errors.
Currently, it only supports 
- Maven
- KarmaJasmine
- Cargo
- java (Only exceptions)
- [https://github.com/crate-ci/typos](https://github.com/crate-ci/typos)

with plans for more.

## Inspiration
For my rust development, I have tried out https://github.com/Canop/bacon that was a good experience because it has a setting where it logs out all error locations for a couple of cargo tools. I liked this and modified my neovim configuration, so I can import this file into my quick fix list. But I also want this behavior for other languages too, so I started with maven.

## Usage
Currently, it is only a small test thing that I am working on. But currently I use it like this
``` bash
mvn clean install |& logana --parser maven --input stdin
```
It will create a output file named ".logana-report".

## Editor support
The editor support alows the editor to parse the ".logana-report" into its now error list.
Plugins are available here:

- neovim [https://github.com/micmine/logana.nvim](https://github.com/micmine/logana.nvim)
- jetbrains [https://github.com/micmine/logana-jetbrains](https://github.com/micmine/logana-jetbrains)
