use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HighlightItem {
    pub icon: String,
    pub label: String,
    pub detail: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SkillsCategories {
    #[serde(rename = "Languages")]
    pub languages: Vec<String>,
    #[serde(rename = "Infrastructure & Cloud")]
    pub infrastructure_and_cloud: Vec<String>,
    #[serde(rename = "Frameworks & Libs")]
    pub frameworks_and_libs: Vec<String>,
    #[serde(rename = "Observability & Tools")]
    pub observability_and_tools: Vec<String>,
}

impl SkillsCategories {
    pub fn categories(&self) -> [(&'static str, &[String]); 4] {
        [
            ("Languages", &self.languages),
            ("Infrastructure & Cloud", &self.infrastructure_and_cloud),
            ("Frameworks & Libs", &self.frameworks_and_libs),
            ("Observability & Tools", &self.observability_and_tools),
        ]
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EducationInfo {
    pub degree: String,
    pub institution: String,
    pub year: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ExperienceItem {
    pub role: String,
    pub company: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ProjectItem {
    pub title: String,
    pub description: String,
    pub tech: Vec<String>,
    pub link: String,
    pub incidents_link: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PortfolioConfig {
    pub name: String,
    pub title: String,
    pub location: String,
    pub open_to_remote: bool,
    pub email: String,
    pub github_url: String,
    pub linkedin_url: String,
    pub resume_url: String,
    pub tagline: String,
    pub about: String,
    pub highlights: Vec<HighlightItem>,
    pub skills: SkillsCategories,
    pub education: EducationInfo,
    pub experience: Vec<ExperienceItem>,
    pub projects: Vec<ProjectItem>,
}

pub const CONFIG_JSON_STR: &str = include_str!("../content/config.json");

pub fn get_config() -> PortfolioConfig {
    serde_json::from_str(CONFIG_JSON_STR).expect("Failed to parse config.json")
}

pub fn get_unique_tech_tags(projects: &[ProjectItem]) -> Vec<String> {
    let mut tags = Vec::new();
    for p in projects {
        for t in &p.tech {
            if !tags.contains(t) {
                tags.push(t.clone());
            }
        }
    }
    tags
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_loads_and_project_count() {
        let config = get_config();
        let raw_value: serde_json::Value =
            serde_json::from_str(CONFIG_JSON_STR).expect("parse raw json");
        let raw_projects = raw_value
            .get("projects")
            .and_then(|p| p.as_array())
            .expect("projects array");

        assert_eq!(config.projects.len(), raw_projects.len());
        assert!(!config.projects.is_empty());
        assert_eq!(config.name, "Odunayo Abraham");
    }

    #[test]
    fn test_unique_tags_ordered_and_deduplicated() {
        let config = get_config();
        let tags = get_unique_tech_tags(&config.projects);

        // Verify uniqueness
        let mut seen = std::collections::HashSet::new();
        for tag in &tags {
            assert!(seen.insert(tag), "Duplicate tag found: {}", tag);
        }

        // Verify that tags follow the appearance order in config.projects
        let mut expected_order = Vec::new();
        for p in &config.projects {
            for t in &p.tech {
                if !expected_order.contains(t) {
                    expected_order.push(t.clone());
                }
            }
        }
        assert_eq!(tags, expected_order);
    }
}
