# HSR Gift Redeemer CLI

This is a CLI tool to redeem gift codes for Honkai Star Rail. It's written in Rust and uses the [reqwest](https://docs.rs/reqwest) library to make HTTP requests.

At the current moment it only has support for the Asia server backend, but it should be easy to add support for other servers by changing the URL in the code.
and writing an enum for backend types.

## Why?

Why not? Plus it lets you redeem gift codes very quickly without having to go to the website and do it manually, and even script it if you want to.

Why Rust? Because why not? If you'd like to rewrite this in a language you think is "saner", feel free. The code is open and the logic is very simple (Authentation from cookies, HTTP query parameters, and JSON response parsing.)

## TODOs

- [ ] Add support for other regions
- [x] Add support for multiple gift codes (should be easy to do with iterators)
- [x] Add support for reading gift codes from a file (Use stdin for this and pipe file to the program)
- [ ] Read cookies from a file instead of using an environment variable or command line option, or even from the browser directly
- [ ] Read UID from a file

## Building and installing

To build the project, you need to have Rust installed. You can install it from [rustup.rs](https://rustup.rs/).

After installing Rust, you can build and install the project by running the following command:

```sh
cargo install --path .
```

This will build the project and install the binary in the `~/.cargo/bin` directory. Make sure that directory is in your `PATH` environment variable.


## Usage

First, you need to get the cookie header from your browser. You can do this by going to the [HSR redeemer page](https://hsr.hoyoverse.com/gift) and logging in. Then, open the developer tools and go to the network tab. Click on the network request tabs, and then look for the headers of the request. You should see a `Cookie` header with a long string of characters. Copy that string and use it as an argument for the `--cookie` option or define it as an environment variable called `COOKIE`.

After getting the cookie, you can redeem a gift code by running the following command:

```sh
hsr-redeemer --cookie <cookie> --uid <UID> <gift-code>
```

You can also use multiple arguments for gift codes or pipe them from a file:

```sh
hsr-redeemer --cookie <cookie> --uid <UID> <gift-code1> <gift-code2> <gift-code3>
```

```sh
cat gift-codes.txt | hsr-redeemer --cookie <cookie> --uid <UID>
# or
hsr-redeemer --cookie <cookie> --uid <UID> < gift-codes.txt
```

Here's an example of a shell script wrapper I use to redeem gift codes (uses [konpeito](https://github.com/tau-OS/konpeito) for secrets management inside the keyring):
```sh
#!/bin/bash

exec env GAME_UID="$(konp get hsr_uid)" hsr-gifter --cookie="$(konp get hsr_cookie)" $@
```

Replace `<cookie>` with the cookie string you copied and `<gift-code>` with the gift code you want to redeem, or omit the `--cookie` option and set the `COOKIE` environment variable instead.

