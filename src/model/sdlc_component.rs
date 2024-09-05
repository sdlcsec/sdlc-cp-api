use std::collections::HashMap;

use schemars::JsonSchema;
use sdlc_cp_api_macro::RegisterSchema;
use uuid::Uuid;

#[derive(Debug, Clone, JsonSchema, RegisterSchema)]
pub enum SDLCComponent {
    Project(Project),   
    Unmanaged(Unmanaged),
}

#[derive(Debug, Clone, JsonSchema)]
pub struct Unmanaged {
    pub id: Uuid,
    pub name: String,
    pub repository_url: Option<String>,
    pub package_url: Option<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, JsonSchema)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub repository_url: Option<String>,
    pub owner: Option<String>,
    pub components: Vec<SDLCComponent>,
}