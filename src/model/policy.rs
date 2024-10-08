use std::time::Duration;
use std::collections::HashMap;

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sdlccp_api_macro::RegisterSchema;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, JsonSchema, RegisterSchema, ToSchema)]
pub struct Policy {
    pub id: String,
    pub name: String,
    pub rules: Vec<PolicyRule>,
    pub parent_policies: Vec<String>, // Todo: This is currently IDs of parent policies, should this be a Vec<Uuid>, should this be some other way of referencing?
    pub applies_to: Vec<String>, // Todo: This currently is Phase names this policy applies to. Should this be a Vec<Phase>? Should this be some other way of referencing?
}

#[derive(Debug, Clone, JsonSchema, ToSchema)]
pub enum PolicyRule {
    MaxAge(Duration),
    ApprovedIdentities(Vec<String>),
    RequiredClaims(HashMap<String, String>),
    VulnerabilityThreshold(VulnerabilityLevel, u32),
}

#[derive(Debug, Clone, PartialEq, PartialOrd, JsonSchema, ToSchema, Serialize, Deserialize)]
pub enum VulnerabilityLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl Policy {
    pub fn new(name: String, applies_to: Vec<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            rules: Vec::new(),
            parent_policies: Vec::new(),
            applies_to,
        }
    }

    pub fn add_rule(&mut self, rule: PolicyRule) {
        self.rules.push(rule);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, RegisterSchema, ToSchema)]
pub struct Vulnerability {
    pub id: String,
    pub severity: VulnerabilityLevel,
    pub description: String,
    pub discovered_at: DateTime<Utc>,
}