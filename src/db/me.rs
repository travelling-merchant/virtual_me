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
    let data: ProtoTypeMe = ProtoTypeMe {
        no_money: 0,
        description: "lol".to_string(),
        current_crisis: "coffe cup empty".to_string(),
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
            field_one: "Money".to_string(),
            field_two: "Description".to_string(),
            field_three: "CRY ABOUT IT".to_string(),
        }
    }
}
