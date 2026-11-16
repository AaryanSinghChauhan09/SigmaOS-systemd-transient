// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-systemd-transient: systemd transient component model.
//!
//! Real implementation: transients, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transient {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemdTransientError {
    EmptyTransientName,
    DuplicateTransient(String),
    EmptyEventSubject,
}

impl fmt::Display for SystemdTransientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyTransientName => write!(f, "transient name must not be empty"),
            Self::DuplicateTransient(id) => write!(f, "duplicate transient {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for SystemdTransientError {}

pub struct SystemdTransient {
    pub name: String,
    transients: BTreeMap<String, Transient>,
    events: BTreeMap<String, Event>,
}

impl SystemdTransient {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            transients: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_transient(&mut self, item: Transient) -> Result<(), SystemdTransientError> {
        if item.name.trim().is_empty() {
            return Err(SystemdTransientError::EmptyTransientName);
        }
        let id = item.id.clone();
        if self.transients.contains_key(&id) {
            return Err(SystemdTransientError::DuplicateTransient(id.clone()));
        }
        self.transients.insert(id, item);
        Ok(())
    }

    pub fn transient_count(&self) -> usize {
        self.transients.len()
    }

    pub fn get_transient(&self, id: &str) -> Option<&Transient> {
        self.transients.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), SystemdTransientError> {
        if event.subject.trim().is_empty() {
            return Err(SystemdTransientError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_transient(id: &str, name: &str) -> Transient {
        Transient {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = SystemdTransient::new("SystemdTransient");
        assert_eq!(m.name, "SystemdTransient");
        assert_eq!(m.transient_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_transient() {
        let mut m = SystemdTransient::new("SystemdTransient");
        m.add_transient(sample_transient("a1", "Alpha")).unwrap();
        assert_eq!(m.transient_count(), 1);
        assert!(m.get_transient("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = SystemdTransient::new("SystemdTransient");
        assert!(m.add_transient(sample_transient("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = SystemdTransient::new("SystemdTransient");
        m.add_transient(sample_transient("x", "One")).unwrap();
        assert!(m.add_transient(sample_transient("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = SystemdTransient::new("SystemdTransient");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
