#!/bin/sh

cargo build --release &&
  mv ./target/release/navi-gator ./
