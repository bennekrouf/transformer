
use crate::models::{Endpoint, Parameter, Response};

// Helper function to determine if the input is singular or plural
fn is_singular(word: &str) -> bool {
    !word.ends_with('s')
}

// Helper function to generate the plural form of a word if needed
fn pluralize(word: &str) -> String {
    if is_singular(word) {
        format!("{}s", word) // Simple pluralization; might need more logic for irregular forms
    } else {
        word.to_string()
    }
}

pub fn generate_endpoints(order_word: &str) -> Vec<Endpoint> {
    // Determine if the order_word is singular or plural
    let singular_order_word = if is_singular(order_word) {
        order_word.to_string()
    } else {
        // Handle case where input is plural and convert to singular
        order_word.trim_end_matches('s').to_string()
    };

    // Plural form for consistency in endpoint paths and descriptions
    let plural_order_word = pluralize(&singular_order_word);

    vec![
        Endpoint {
            path: format!("get_a_list_of_{}", plural_order_word).to_string(),
            description: format!("Retrieve a list of last {}", plural_order_word).to_string(),
            parameters: vec![], // No parameters
            response: Response {
                r#type: format!("List of {}", plural_order_word).to_string(),
            },
        },
        Endpoint {
            path: format!("create_new_{}", singular_order_word).to_string(),
            description: format!("Create a new {}", singular_order_word).to_string(),
            parameters: vec![Parameter {
                name: format!("{}_name", singular_order_word).to_string(),
                r#type: "string".to_string(),
                description: Some(format!("Name of the {} to create", singular_order_word).to_string()),
                required: true,
            }],
            response: Response {
                r#type: format!("{} confirmation", singular_order_word).to_string(),
            },
        },
        Endpoint {
            path: format!("update_{}", singular_order_word).to_string(),
            description: format!("Update an existing {}", singular_order_word).to_string(),
            parameters: vec![Parameter {
                name: "id".to_string(),
                r#type: "string".to_string(),
                description: Some(format!("ID of the {} to update", singular_order_word).to_string()),
                required: true,
            }],
            response: Response {
                r#type: format!("Update confirmation for {}", singular_order_word).to_string(),
            },
        },
        Endpoint {
            path: format!("delete_{}", singular_order_word).to_string(),
            description: format!("Delete a {}", singular_order_word).to_string(),
            parameters: vec![Parameter {
                name: "id".to_string(),
                r#type: "string".to_string(),
                description: Some(format!("ID of the {} to delete", singular_order_word).to_string()),
                required: true,
            }],
            response: Response {
                r#type: format!("Deletion confirmation for {}", singular_order_word).to_string(),
            },
        },
        Endpoint {
            path: format!("send_an_email_related_to_{}", singular_order_word).to_string(),
            description: format!("Send an email related to a {}", singular_order_word).to_string(),
            parameters: vec![Parameter {
                name: "email".to_string(),
                r#type: "string".to_string(),
                description: Some(format!("Email address to send the notification to about the {}", singular_order_word).to_string()),
                required: true,
            }],
            response: Response {
                r#type: format!("Email sent confirmation for {}", singular_order_word).to_string(),
            },
        },
    ]
}

