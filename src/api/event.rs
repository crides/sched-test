use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use chrono::{DateTime, Local, Duration};
use rlua::prelude::*;

pub enum Priority {
    VeryHigh,
    High,
    NotSoHigh,
    MediumHigh,
    Medium,
    MediumLow,
    NotSoLow,
    Low,
    VeryLow,
}

pub struct Interval {
    start: DateTime<Local>,
    length: Duration,
}

impl Interval {
    pub fn builder() -> IntervalBuilder {
        IntervalBuilder {
            start: None,
            length: None,
            end: None,
        }
    }
}

pub enum IntervalBuildError {
    /// When the caller specifies all 3 constraints, and they don't match
    TooManyConstraints,
    /// When the caller specifies only 1 of the 3 constraints
    NotEnoughConstraints,
}

pub struct IntervalBuilder {
    start: Option<DateTime<Local>>,
    length: Option<Duration>,
    end: Option<DateTime<Local>>,
}

impl IntervalBuilder {
    pub fn start(&mut self, start: DateTime<Local>) -> &mut Self {
        self.start = Some(start);
        self
    }

    pub fn length(&mut self, length: Duration) -> &mut Self {
        self.length = Some(length);
        self
    }

    pub fn end(&mut self, end: DateTime<Local>) -> &mut Self {
        self.end = Some(end);
        self
    }

    pub fn build(&self) -> Result<Interval, IntervalBuildError> {
        match (self.start, self.length, self.end) {
            (Some(s), Some(l), Some(e)) => {
                if s + l != e {
                    Err(IntervalBuildError::TooManyConstraints)
                } else {
                    Ok(Interval {
                        start: s,
                        length: l,
                    })
                }
            },
            (Some(s), None, Some(e)) => {
                Ok(Interval {
                    start: s,
                    length: e - s,
                })
            },
            (Some(s), Some(l), None) => {
                Ok(Interval {
                    start: s,
                    length: l,
                })
            },
            (None, Some(l), Some(e)) => {
                Ok(Interval {
                    start: e - l,
                    length: l,
                })
            },
            _ => Err(IntervalBuildError::NotEnoughConstraints),
        }
    }
}

pub struct EventCommon<'lua> {
    name: String,
    priority: Priority,
    handlers: HashMap<String, LuaFunction<'lua>>,
    props: HashMap<String, String>,
    finished: bool,
}

impl<'lua> EventCommon<'lua> {
    pub fn new(name: String, priority: Priority) -> Self {
        Self {
            name,
            priority,
            handlers: HashMap::new(),
            props: HashMap::new(),
            finished: false,
        }
    }

    pub fn set_handler<S: AsRef<str>>(&mut self, key: S, f: LuaFunction<'lua>) {
        self.handlers.insert(key.as_ref().into(), f);
    }

    pub fn set_prop<S: AsRef<str>>(&mut self, key: S, val: String) {
        self.props.insert(key.as_ref().into(), val);
    }

    pub fn finish(&mut self) {
        self.finished = true;
    }
}

pub struct Event<'lua> {
    inner: EventCommon<'lua>,
    interval: Interval,
}

impl<'lua> Event<'lua> {
    pub fn new(inner: EventCommon<'lua>, interval: Interval) -> Self {
        Self {
            inner,
            interval,
        }
    }
}

pub struct Task<'lua> {
    inner: EventCommon<'lua>,
    sessions: Vec<Interval>,
}

impl<'lua> Task<'lua> {
    pub fn new(inner: EventCommon<'lua>) -> Self {
        Self {
            inner,
            sessions: Vec::new(),
        }
    }
}

pub struct Project<'lua> {
    inner: EventCommon<'lua>,
    subtasks: Vec<Task<'lua>>,
}

impl<'lua> Project<'lua> {
    pub fn new(inner: EventCommon<'lua>) -> Self {
        Self {
            inner,
            subtasks: Vec::new(),
        }
    }
}

pub enum EventType<'lua> {
    Event(Event<'lua>),
    Task(Task<'lua>),
    Project(Project<'lua>),
}
