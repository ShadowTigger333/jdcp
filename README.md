# Josh's D&D Character Protocol (JDCP)

I wanted to learn Rust with binary protocols similar to OT protocols I've worked with. The Nom crate seemed like a good place as any to start.

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

## How to use the client<>server
```bash
#Server
cargo run --bin server -- 12345

#Client
cargo run --bin client
```