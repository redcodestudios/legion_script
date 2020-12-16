# legion_script

An extensible scripting system for Legion _ECS_.

## Introduction

This is result of a year of research and contect with Rust, Amethyst and Legion community. The main goal of this _POC_ is to add scripting capabilities to Legion _ECS_. Currently it supports ```Python``` as scripting langauge but it has an architecture extensible to other languages. This work was by the [Amethyst Scripting RFC](https://github.com/amethyst/rfcs/blob/master/0001-scripting.md).

## Running

**This is a _POC_ and no effort was made to add multiplatform features. Therefore this crate only works on linux platforms**

To run this project you need ```Rust``` and ```Python 3.7``` installed. After installing both run this command inside the folder:

```$ cargo run --example=hello```

This will run the example hello showing how to use.
