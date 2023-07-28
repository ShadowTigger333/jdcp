# Josh's D&D Character Protocol (JDCP)

I wanted to learn Rust with binary protocols similar to OT protocols I've worked with. Based on [this list of parsers](https://lib.rs/parsing), the Nom crate seemed like a good place as any to start. 

## The Protocol
The JDCP is loosely based on CIP. Here's what it looks like broken down (I'll break it down more later):
Protocol Preamble | Message Type | Character Name | Info Type | Data Size | Data
---|---|---|---|---|---
"jdcp-" | 1 u8 | \x00 ended ascii string | 1 u8 | 1u16 (LE) | Data... 

Here's the data field broken down a little
Data Type | Bytes | Struct
---|---|---
Stats | 6u8 | Stats(str, dex, con, wis, int, cha)
Age | 1u16 (LE) | 0 - 65535
Class | 1u8  | Class Enum
Race | 1u8 | Race Enum
Level | 1u8 | 0 - 255
Health Points | 2u8 | HP(current, max)

Class and Race Enum Values:
Class | Value | Race
---|---|---
ARTIFICER | 1 | DWARF
BARBARIAN | 2 | ELF
BARD | 3 | GNOME
BLOODHUNTER | 4 | HALFELF
CLERIC | 5 | HALFLING
DRUID | 6 | HALFORC
FIGHTER | 7 | HUMAN
MONK | 8 | ORC
PALADIN | 9 | TIEFLING
RANGER | 10 |
ROGUE | 11 |
SORCERER | 12 |
WARLOCK | 13 |
WIZARD | 14 |

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


### TODO
1. Add a separate character object
1. Add doc comments
1. Add a new field for arbitrary length "description"
