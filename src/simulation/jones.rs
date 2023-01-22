use std::collections::{HashMap, HashSet};

use crate::automaton::determ_autom::{Autom, Transition};
use crate::simulation::config::{Config, DeltaConfig, StrippedConfig, strip_config, make_delta_config};
use crate::parser::ast::{Readable, Input};

