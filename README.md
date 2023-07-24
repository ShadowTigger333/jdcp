# Josh's D&D Character Protocol (JDCP)

I wanted to learn Rust with binary protocols similar to OT protocols I've worked with. Based on [this list of parsers](https://lib.rs/parsing), the Nom crate seemed like a good place as any to start. 

## The Protocol
The JDCP is loosely based on CIP. Here's what it looks like broken down (I'll break it down more later):
Protocol Preamble | Message Type | Character Name | Info Type | Data Size | Data
---|---|---|---|---|---
"jdcp-" | 1 u8 | \x00 ended ascii string | 1 u8 | 1u16 (LE) | Data... 

## Notes
I'm trying to learn a lot and jumping back and forth diving into the "Rustian" ways of TDD, abstraction, and error handling while also learning about lifetimes, streaming data, and ownership.

#### Thoughts
* While the compiler complains a lot, it's actually reasonably helpful. It's been a minute since I worked in C, but I definitely feel good so far
* Lifetimes make sense theoretically as I read about them; however, I will need more practice to better understand them experientially.
* ~~Nom's cond parser seems like it could be powerful, but I think I'm missing a key part of how to conditionally map decoding data based on both the length and type variables. It returns a successful run of the "parser" so the Alt tag doesn't pair well with it.~~ Update: I am using a combination of the tuple and validate parsers to do this now.

## How to use the client<>server
```bash
#Server
cargo run --bin server -- 12345

#Client
cargo run --bin client
```
