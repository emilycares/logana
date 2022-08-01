# logana

A build log analysis tool so that your PDE (Personal Development Environment) can have a list of files that contain errors.
Currently, it only supports maven, with plans for more.

## Inspiration
For my rust development, I have tried out https://github.com/Canop/bacon that was a good experience because it has a setting where it logs out all error locations for a couple of cargo tools. I liked this and modified my neovim configuration, so I can import this file into my quick fix list. But I also want this behavior for other languages too, so I started with maven.

## Usage
Currently, it is only a small test thing that I am working on. But currently I use it like this
``` bash
mvn clean install | ~/Documents/rust/logana/target/debug/logana > .logana-report
```
