# Listbotto

A mini music server controlled via API calls written in [Rust][]

## How does it work?

It is a combined music indexer and player, you can send API requests via
REST and receive JSON responses.

## What is it for?

It's a remote-controlled jukebox that you can control via anything that speaks
HTTP. The new intent is to be able to fit this daemon onto a small RaspberryPi
for my home office so it can be easily controlled to play random music for me
or maybe a party.

## Requirements

* Rust 1.7.0 or later to build, binary releases run on their own without the
  need for a Rust runtime.

## License

Copyright 2016 Aaron Kalin

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

[Rust]: https://www.rust-lang.org/
