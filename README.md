# camtcsv

**state: version 0.0.0 ;) (see `Cargo.toml`)**

Reads bank transactions from a [camt (cash management)][wikipedia] csv file to rust structs.

It offers a function that takes a path to a camt csv from file and returns a vector of transactions (or an error). It's probably NOT compatible with every camt csv file as it currently expects the german header files as used by my bank. This is because it was simple to do with `serde`. The "Transaction" struct is currently a simple representation without references or anything.


Note that the provided example needs a valid camt csv file and currently I do not provide one.

[wikipedia]: https://de.wikipedia.org/wiki/Camt-Format (Cash Management on Wikipedia)
