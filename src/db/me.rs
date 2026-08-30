use leptos::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProtoTypeMe {
    pub no_money: u8,
    pub description: String,
    pub current_crisis: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProtoTypeMeNames {
    pub field_one: String,
    pub field_two: String,
    pub field_three: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Field {
    pub label: String,
    pub value: String,
}
#[server]
pub async fn provide_user_data() -> Result<Vec<Field>, ServerFnError> {
    let mut data_as_vec: Vec<Field> = Vec::new();
    let descriptions: ProtoTypeMeNames = ProtoTypeMeNames::default();
    let masteries = "Masteries: German (C2), English (C1), Software Engineering (Tier 3), Linux (Tier 3), C# (Tier 3), OOP (Tier 2), Git (Tier 2), Rust (Tier 1), Economics (Tier 2)";
    let data: ProtoTypeMe = ProtoTypeMe {
        no_money: 2,
        description: "Software Engieer".to_string(),
        current_crisis: masteries.to_string(),
    };
    let field: Field = Field {
        label: descriptions.field_one,
        value: data.no_money.to_string(),
    };
    data_as_vec.push(field);
    let field: Field = Field {
        label: descriptions.field_two,
        value: data.description,
    };
    data_as_vec.push(field);
    let field: Field = Field {
        label: descriptions.field_three,
        value: data.current_crisis,
    };
    data_as_vec.push(field);
    Ok(data_as_vec)
}
impl Default for ProtoTypeMeNames {
    fn default() -> Self {
        ProtoTypeMeNames {
            field_one: "Level".to_string(),
            field_two: "Class".to_string(),
            field_three: "Masteries".to_string(),
        }
    }
}

/// A single row of the strategic roadmap table.
#[derive(Clone, Debug)]
pub struct Sector {
    pub name: &'static str,
    pub current: &'static str,
    pub target: &'static str,
    pub status: &'static str,
    pub trend: &'static str,
}

/// The strategic roadmap. Add/reorder rows here and the table updates.
pub fn strategic_sectors() -> Vec<Sector> {
    vec![
        Sector {
            name: "Embedded (ESP32/RTOS)",
            current: "Uninitialized (Sealed Box)",
            target: "Independent Firmware Dev",
            status: "🌑",
            trend: "🌑",
        },
        Sector {
            name: "Linux Systems (WM/DE)",
            current: "Power User (Arch/DWL)",
            target: "Compositor/WM Author",
            status: "🌑",
            trend: "🌑",
        },
        Sector {
            name: "Rust Ecosystem",
            current: "Revisit the Book",
            target: "no_std & Wayland Proficient",
            status: "🌑",
            trend: "🌑",
        },
        Sector {
            name: "Robotics",
            current: "Conceptual",
            target: "Applied ROS2/Kinematics",
            status: "🌑",
            trend: "🌑",
        },
        Sector {
            name: "Mandarin",
            current: "Beginner",
            target: "Basic Conversation",
            status: "🌘",
            trend: "🌘",
        },
        Sector {
            name: "Japanese",
            current: "Beginner",
            target: "Basic Conversation",
            status: "🌘",
            trend: "🌘",
        },
        Sector {
            name: "Backup Insurance",
            current: "None",
            target: "Bio/History/Physics/Math ETH Exam ready",
            status: "🌑",
            trend: "🌑",
        },
    ]
}
