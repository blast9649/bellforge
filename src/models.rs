//! Core domain models for bellforge
//!
//! Used by the Template Editor (PR 2) and the Session Runner (PR 3+).

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A complete, savable workout template.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkoutTemplate {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub default_weight_kg: Option<f32>,

    /// Default rest between individual exercises inside a round
    pub rest_between_exercises_s: u32,
    /// Default rest after completing a full round
    pub rest_between_rounds_s: u32,

    /// The sequence of steps (supports Exercise, Rest, and top-level Repeat)
    pub flow: Vec<FlowItem>,
}

impl WorkoutTemplate {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            description: None,
            tags: vec!["kettlebell".to_string()],
            default_weight_kg: Some(24.0),
            rest_between_exercises_s: 45,
            rest_between_rounds_s: 90,
            flow: Vec::new(),
        }
    }

    /// Very rough but useful estimate for the editor.
    /// Does not account for actual work time (user-dependent).
    pub fn estimated_duration_seconds(&self) -> u32 {
        let mut total = 0u32;
        let global_ex = self.rest_between_exercises_s;
        let global_round = self.rest_between_rounds_s;

        for item in &self.flow {
            total += item.estimated_rest_seconds(global_ex, global_round);
        }

        // Assume ~25 seconds average "work" per exercise set (very rough)
        let exercise_sets: u32 = self
            .flow
            .iter()
            .map(|i| i.count_exercise_sets())
            .sum();

        total + exercise_sets * 25
    }

    pub fn estimated_duration_minutes(&self) -> u32 {
        (self.estimated_duration_seconds() + 30) / 60
    }

    /// Returns a human-friendly summary for the dashboard.
    pub fn summary(&self) -> String {
        let steps = self.flow.len();
        let rounds = self.count_rounds();
        format!("{} steps • ~{} min • {} rounds", steps, self.estimated_duration_minutes(), rounds)
    }

    fn count_rounds(&self) -> u32 {
        self.flow
            .iter()
            .filter_map(|item| {
                if let FlowItem::Repeat { count, .. } = item {
                    Some(*count)
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(1)
    }
}

/// One atomic step in a workout template.
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
    /// Simple top-level repeat (no deep nesting in v1)
    Repeat {
        count: u32,
        items: Vec<FlowItem>,
    },
}

impl FlowItem {
    pub fn display_name(&self) -> String {
        match self {
            FlowItem::Exercise { name, reps, sets, .. } => {
                if *sets > 1 {
                    format!("{} — {} × {} reps", name, sets, reps)
                } else {
                    format!("{} — {} reps", name, reps)
                }
            }
            FlowItem::Rest { label, duration_s } => {
                format!("{} ({}s)", label, duration_s)
            }
            FlowItem::Repeat { count, items } => {
                format!("Repeat ×{} ({} steps)", count, items.len())
            }
        }
    }

    /// Returns the rest that should be used after this item, respecting per-item overrides.
    pub fn effective_rest_after(
        &self,
        global_exercise_rest: u32,
        global_round_rest: u32,
    ) -> u32 {
        match self {
            FlowItem::Exercise { rest_after_exercise_s, .. } => {
                rest_after_exercise_s.unwrap_or(global_exercise_rest)
            }
            FlowItem::Rest { duration_s, .. } => *duration_s,
            FlowItem::Repeat { .. } => global_round_rest,
        }
    }

    /// Rough rest time contributed by this item (used for time estimation).
    pub fn estimated_rest_seconds(&self, global_ex: u32, global_round: u32) -> u32 {
        match self {
            FlowItem::Exercise { rest_after_set_s, rest_after_exercise_s, sets, .. } => {
                let per_set = rest_after_set_s.unwrap_or(global_ex);
                let after_ex = rest_after_exercise_s.unwrap_or(global_ex);
                // Between sets + final rest after the exercise
                per_set * (sets.saturating_sub(1)) + after_ex
            }
            FlowItem::Rest { duration_s, .. } => *duration_s,
            FlowItem::Repeat { count, items } => {
                // Very approximate: repeat the inner items + round rest
                let inner: u32 = items.iter().map(|i| i.estimated_rest_seconds(global_ex, global_round)).sum();
                inner * count + global_round * (count.saturating_sub(1))
            }
        }
    }

    pub fn count_exercise_sets(&self) -> u32 {
        match self {
            FlowItem::Exercise { sets, .. } => *sets,
            FlowItem::Repeat { count, items } => {
                items.iter().map(|i| i.count_exercise_sets()).sum::<u32>() * count
            }
            _ => 0,
        }
    }

    // Convenience constructors
    pub fn exercise(name: impl Into<String>, reps: u32, sets: u32) -> Self {
        FlowItem::Exercise {
            name: name.into(),
            reps,
            weight_kg: None,
            sets,
            rest_after_set_s: None,
            rest_after_exercise_s: None,
        }
    }

    pub fn rest(duration_s: u32, label: impl Into<String>) -> Self {
        FlowItem::Rest {
            duration_s,
            label: label.into(),
        }
    }
}
