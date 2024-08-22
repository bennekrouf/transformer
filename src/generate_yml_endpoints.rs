
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

// Function to generate the endpoint for listing items
fn generate_list_endpoint(domain_word: &str) -> Endpoint {
    let plural_order_word = pluralize(domain_word);

    Endpoint {
        path: format!("get_a_list_of_{}", plural_order_word),
        description: format!("Retrieve a list of last {}", plural_order_word),
        parameters: vec![], // No parameters
        response: Response {
            r#type: format!("List of {}", plural_order_word),
        },
    }
}

// Function to generate the endpoint for creating a new item
fn generate_create_endpoint(domain_word: &str) -> Endpoint {
    let singular_order_word = if is_singular(domain_word) {
        domain_word.to_string()
    } else {
        domain_word.trim_end_matches('s').to_string()
    };

    Endpoint {
        path: format!("create_new_{}", singular_order_word),
        description: format!("Create a new {}", singular_order_word),
        parameters: vec![Parameter {
            name: format!("{}_name", singular_order_word),
            r#type: "string".to_string(),
            description: Some(format!("Name of the {} to create", singular_order_word)),
            required: true,
        }],
        response: Response {
            r#type: format!("{} confirmation", singular_order_word),
        },
    }
}

// Function to generate the endpoint for updating an item
fn generate_update_endpoint(domain_word: &str) -> Endpoint {
    let singular_order_word = if is_singular(domain_word) {
        domain_word.to_string()
    } else {
        domain_word.trim_end_matches('s').to_string()
    };

    Endpoint {
        path: format!("update_{}", singular_order_word),
        description: format!("Update an existing {}", singular_order_word),
        parameters: vec![Parameter {
            name: "id".to_string(),
            r#type: "string".to_string(),
            description: Some(format!("ID of the {} to update", singular_order_word)),
            required: true,
        }],
        response: Response {
            r#type: format!("Update confirmation for {}", singular_order_word),
        },
    }
}

// Function to generate the endpoint for deleting an item
fn generate_delete_endpoint(domain_word: &str) -> Endpoint {
    let singular_order_word = if is_singular(domain_word) {
        domain_word.to_string()
    } else {
        domain_word.trim_end_matches('s').to_string()
    };

    Endpoint {
        path: format!("delete_{}", singular_order_word),
        description: format!("Delete a {}", singular_order_word),
        parameters: vec![Parameter {
            name: "id".to_string(),
            r#type: "string".to_string(),
            description: Some(format!("ID of the {} to delete", singular_order_word)),
            required: true,
        }],
        response: Response {
            r#type: format!("Deletion confirmation for {}", singular_order_word),
        },
    }
}

// Function to generate the endpoint for sending an email related to an item
fn generate_email_endpoint(domain_word: &str) -> Endpoint {
    let singular_order_word = if is_singular(domain_word) {
        domain_word.to_string()
    } else {
        domain_word.trim_end_matches('s').to_string()
    };

    Endpoint {
        path: format!("send_an_email_related_to_{}", singular_order_word),
        description: format!("Send an email related to a {}", singular_order_word),
        parameters: vec![Parameter {
            name: "email".to_string(),
            r#type: "string".to_string(),
            description: Some(format!("Email address to send the notification to about the {}", singular_order_word)),
            required: true,
        }],
        response: Response {
            r#type: format!("Email sent confirmation for {}", singular_order_word),
        },
    }
}

// Main function to generate all endpoints
pub fn generate_endpoints(domain_word: &str) -> Vec<Endpoint> {
    vec![
        generate_list_endpoint(domain_word),
        generate_create_endpoint(domain_word),
        generate_update_endpoint(domain_word),
        generate_delete_endpoint(domain_word),
        generate_email_endpoint(domain_word),
    ]
}

