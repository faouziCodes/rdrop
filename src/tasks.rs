use crate::{drop_tables::DropTables, tasker::Task};
use std::collections::HashMap;

pub const TASKS_TABLE: HashMap<String, Box<dyn Task>> =
    HashMap::from([("DropTables".into(), Box::new(DropTables::new()))]);
