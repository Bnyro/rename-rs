# rename-rs
Command line tool to bulk rename files recursively. It's fast!

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
