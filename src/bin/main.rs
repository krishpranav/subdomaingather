extern crate sub;
use clap::{App, Arg};
use futures::stream::StreamExt;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use sub::error::Result;
use sub::{CleanExt, PostProcessor, Runner};