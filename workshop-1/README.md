# Rust workshop 1 - Rust 101

This is the first session of the rust workshop, and is meant to be a quick introduction to the mechanics of the Rust language. It will also involve a small CLI project at the end as a practice for the participants.

## Prerequisite

To begin the workshop, you have to make sure to install Rust. You can install rust by going to the rust website and install the latest rust version.

## Content

### Part 1: Hello world

First of all let's go through the boring part: learning about the components of the language.

There's the standard hello world exercise:

```
```


### Part 2: Diving into ownership

In rust there's a concept of ownership & borrowing.


### Part 3: The Internet!

Finally we are getting to something that's closer to day to day usage of programming languages!

We're going to need some libraries to help us call and api and do serialization & deserialization for us. To start, let's install the relevant dependency.

```toml
# Add the following lines to the dependencies section of your cargo.toml
reqwest = { version = "0.12", features = ["json", "blocking"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

What we've done here is essentially install both the dependencies for calling a url & also handle the serialization and deserialization of the data that we get from the api as well. The `reqwest` and `tokio` crates are used for making the http request, whilst the `serde` & `serde_json` crates are used for the serialization and deserialization of the incoming and outgoing payload.

Under the hood, when you attempt to call the api, what's actually happening is actually the standard Http request!


### Part 4: Building a wallet cli

Let's put our learnt knowledge to practice by building a CLI to get the recent transactions from your wallet address.

Begin with getting a RPC to the Ethereum blockchain. You can get a free RPC either via Alchemy or Infura for something a little bit more reliable, or use one that's open to public at [chainlist.org](https://chainlist.org/).

With the RPC url, you can now work

## References

- [Rust by example](https://doc.rust-lang.org/rust-by-example)
- [The rust book](https://doc.rust-lang.org/book/)
- [Too many lists](https://rust-unofficial.github.io/too-many-lists/)
- [Common lifetime misconception](https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md). This is a bonus and is not covered in this workshop's content.