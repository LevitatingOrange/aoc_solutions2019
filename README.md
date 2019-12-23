# My AOC solutions
Written in Haskell, Rust, ... . Powered by Bazel.

# Rationale
I wanted to try and implement the AOC challenges in as many different
programming languages as I can. As the Intcode computer is a big, reoccuring
challenge, I used the language I am most comfortable with; Rust. Moreover, I
encapsulated the Intcode computer into it's own package, so that solutions are only
frontends to the provided library. Because I use multiple programming languages,
and I worked with Bazel in the last few months anyway, it seemed a good fit as
one of it's main selling points are big multilingual monorepos.

# Utils
If you want to use Bazel in your own AOC adventures, look into the `utils`
package. It contains a very simple Python script that will download problem
inputs and provide them as `input_<zero padded day num>` to Bazel rules as a
cached data dependency. Sadly I could not use the default `http_file` rules of
Bazel, because they do not provide authentication via cookies.

You have to use bazel runfiles to access these files in a robust and elegant
manner. There are libraries for Python, Haskell and more that do that for you.
Wherever there is no library, I will (hopefully) implement my own.

# Setup 
* Install Bazel (e.g. via Bazelisk)
* Install Rust via `rustup`
* Install `cargo raze` dependices via `cargo install cargo-vendor cargo-raze`
* Run `cargo-vendor vendor -x && cargo raze` to download cargo dependencies and
  make them available to Bazel
* Put your Advent Of Code session cookie into `util/.session_cookie` so that
  Bazel can download the puzzle inputs. At the moment only one is supported.
  Bazel will cache them for you as long as you don't fiddle aroud with the
  download script and BUILD rules in `utils`. 
* Do `bazel run //<problem>` to execute (e.g. `bazel run //day01`) 