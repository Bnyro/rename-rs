# rename-rs
Command line tool to bulk rename files recursively. It's fast!

## Installation
Download the latest release tarball from the [releases page](https://github.com/Bnyro/rename-rs/releases/latest).
<br />
Extract the binary by running `tar xvf rn-{release version and architecture}.tar.gz`.
<br />
Move the `rn` binary to any directory in your path variable, for example by running `mv rn /usr/local/bin/rn`.
<br />
Congrats! You're now ready to use it in production.

## Basic usage
```
Usage: rmv [OPTIONS] <FROM> <TO>

Arguments:
  <FROM>
  <TO>

Options:
  -r, --recursively
  -d, --directory <DIRECTORY>  [default: .]
  -h, --help                   Print help
  -V, --version                Print version
```

## Example patterns
Change the extension of all `opus` files in the current directory to `ogg`
```bash
rn '*.opus' '$1.ogg'
```

Change the extension of all `svg` files to `xml` recursively
```bash
rn -r '*.svg' '$1.xml'
```

Remove the first whitespace from filenames in the home directory (e.g. `Hello World.txt` -> `HelloWorld.txt`)
```bash
rn --directory '~' '* *' '$1$2'
