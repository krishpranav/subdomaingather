# subdomaingather
A simple tool to gather subdomains from passive sources build in rust

[![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg)](https://forthebadge.com)


# Installation
```
$ git clone https://github.com/krishpranav/subdomaingather
$ cd subdomaingather
$ cargo build --release
$ ./target/release/subdomaingather --version
```

# Usage
```
$ subdomaingather -d hackerone.com
```

**Collecting data only on a specific subdomain**

If you only want to collect results related to a specific subdomain you can use
the `--subs-only` flag. This will cause subdomaingather to run on the actual domain and not
the root domain. Results will be filtered to anything that ends with the input
domain or domains.
```
$ subdomaingather -d api.hackerone.com --subs-only
```

**With a list of domains from a file**

```
$ subdomaingather -f path/to/domains.txt
```

**With a list of domains from stdin**

```
$ subdomaingather < /path/to/domains.txt
```

**Outputting results as they're received**

You can output results as they arrive as opposed to once all sources finish using
the `--flush` flag. You might want to use this flag when you're running subdomaingather on a
small vps without much memory. When this flag is active subdomaingather will not remove duplicates,
so you may want to pipe it through something like `sort -u`.
```
$ subdomaingather -d hackerone.com --flush
```

**Collecting data using paid sources**

If you want to include sources which require API keys, add the `-a` or `-all` flag, for example:
```
$ subdomaingather -d hackerone.com -a
``` 
By default it will just ignore services you don't supply keys for.

You can exclude sources with the `-e` flag
```
$ subdomaingather -d hackerone.com -e Wayback
```

* `info`: General information like how many results each source returned.
* `debug`: Lots and lots of information about what's going on under the hood.
```
$ subdomaingather -d hackerone.com -v info
```

# Credit

- subdomaingather is based heavily on [vita](https://github.com/junnlikestea/vita) by [@junnlikestea](https://github.com/junnlikestea)
