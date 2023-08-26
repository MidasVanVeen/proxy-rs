# proxy-rs - Proxy Checker Tool

proxy_rs is a command-line tool written in Rust for checking the validity and performance of proxy servers. It allows you to quickly assess a list of proxy servers, test their responsiveness, and save the working proxies to an output file.

This project uses ureq, one of the fastest request libraries available for rust.

## Usage

```plain
Usage: proxy_rs [OPTIONS] --file <FILE>

Options:
  -f, --file <FILE>
  -o, --outfile <OUTFILE>  [default: checked-proxies.txt]
  -v, --verbose
  -t, --timeout <TIMEOUT>  [default: 5]
  -r, --retries <RETRIES>  [default: 3]
  -T, --threads <THREADS>  [default: 200]
  -h, --help               Print help
  -V, --version            Print version
```

## Options

- `-f, --file <FILE>`: Specify the path to the input file containing the list of proxy servers to be checked.
- `-o, --outfile <OUTFILE>`: Specify the name of the output file where the working proxy servers will be saved. (Default: checked-proxies.txt)
- `-v, --verbose`: Enable verbose mode, providing more detailed output during the checking process.
- `-t, --timeout <TIMEOUT>`: Set the timeout value (in seconds) for each proxy server check. (Default: 5)
- `-r, --retries <RETRIES>`: Set the number of retries for failed proxy connection attempts. (Default: 3)
- `-T, --threads <THREADS>`: Set the number of threads to use for parallel proxy checking. (Default: 200)
- `-h, --help`: Print the help message detailing the usage of the tool.
- `-V, --version`: Print the version information of the proxy_rs tool.

## Adding Protocols
If your list of proxy servers is in the format host:port, you can use the provided Python script add-protocol.py to add the protocol prefix. The script takes an input file and a protocol as arguments and outputs the modified list to stdout.

### Usage

```sh
./add-protocol.py input-proxies.txt http > output.txt
```
Replace http with the desired protocol.

## Examples
Check a list of proxy servers with the "https" protocol, verbose output, a timeout of 10 seconds, and using 100 threads:

``` sh
proxy_rs -f proxies.txt -p https -v -t 10 -T 100
```

## License
This project is licensed under the [MIT License](https://mit-license.org).

Feel free to contribute, report issues, or suggest improvements on [GitHub](https://github.com/MidasVanVeen/proxy-rs).
