# Blackwood - CycleDeath I
![blackwood](./src/blackwood.jpg "")

## Overview
Blackwood is an advanced artificial intelligence program built on OpenAI technology and ARC architecture. The program's core consists of a Twitter-connected agent and a GPT-4 powered RSS collector. Blackwood aims to simulate the life cycle of individuals to explore the meaning of individual existence, the essence of death, and the role of information in the perpetuity of the individual.

## Get Started
cargo build
cargo add rig-core
cargo run

## Simple example:
Note using #[tokio::main] requires you enable tokio's macros and rt-multi-thread features or just full to enable all features (cargo add tokio --features macros,rt-multi-thread).

You can find more examples each crate's examples (ie. src/examples) directory. More detailed use cases walkthroughs are regularly published on our Dev.to Blog and added to Rig's official documentation (docs.rig.rs).
