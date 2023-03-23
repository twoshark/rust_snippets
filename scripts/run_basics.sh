#!/bin/sh

rustc tutorials/rust_gentle_intro/basics.rs \
&& ./basics README.md cliArgsTest2 \
&& rm basics
