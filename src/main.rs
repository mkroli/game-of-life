/*
 * Copyright 2016 Michael Krolikowski
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#[macro_use] extern crate clap;
extern crate rand;
extern crate rustty;

use clap::{Arg, App};

mod game_of_life;
pub use game_of_life::*;

fn main() {
    let app = App::new("game-of-life")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("pause")
                .short("p")
                .long("pause")
                .value_name("milliseconds")
                .help("pause between steps in milliseconds")
                .takes_value(true)
        );
    let matches = app.get_matches();
    let pause = value_t!(matches, "pause", u64).unwrap_or(0);

    match GameOfLife::start(pause) {
        Err(e) => println! ("Error: {}", e),
        _ => ()
    }
}
