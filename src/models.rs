//! Core domain models for bellforge (PR 1 stub)
//!
//! These will be expanded in PR 2 (template editor) and used throughout.
//!
//! We intentionally allow dead_code here because the models are not yet
//! wired into the UI (they will be used starting in PR 2).

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkoutTemplate {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub default_weight_kg: Option<f32>,

    pub rest_between_exercises_s: u32,
    pub rest_between_rounds_s: u32,

    pub flow: Vec<FlowItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FlowItem {
    Exercise {
        name: String,
        reps: u32,
        weight_kg: Option<f32>,
        sets: u32,
        rest_after_set_s: Option<u32>,
        rest_after_exercise_s: Option<u32>,
    },
    Rest {
        duration_s: u32,
        label: String,
    },
    Repeat {
        count: u32,
        items: Vec<FlowItem>,
    },
}

// Placeholder for future session models
// pub struct CompletedSession { ... }
