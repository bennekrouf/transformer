
use crate::models::{Endpoint, Parameter, Response};

pub fn generate_endpoints() -> Vec<Endpoint> {
    vec![
        Endpoint {
            path: "get_a_list_of_last_orders".to_string(),
            description: "Retrieve a list of last orders".to_string(),
            parameters: vec![], // No parameters
            response: Response {
                r#type: "List of orders".to_string(),
            },
        },
        Endpoint {
            path: "create_new_order".to_string(),
            description: "Create a new order".to_string(),
            parameters: vec![Parameter {
                name: "tenant_name".to_string(),
                r#type: "string".to_string(),
                description: Some("Name of the tenant to create".to_string()),
                required: true,
            }],
            response: Response {
                r#type: "Order confirmation".to_string(),
            },
        },
        Endpoint {
            path: "update_order".to_string(),
            description: "Update an existing order".to_string(),
            parameters: vec![Parameter {
                name: "id".to_string(),
                r#type: "string".to_string(),
                description: Some("ID of the entity to update".to_string()),
                required: true,
            }],
            response: Response {
                r#type: "Update confirmation".to_string(),
            },
        },
        Endpoint {
            path: "delete_order".to_string(),
            description: "Delete an order".to_string(),
            parameters: vec![Parameter {
                name: "id".to_string(),
                r#type: "string".to_string(),
                description: Some("ID of the entity to delete".to_string()),
                required: true,
            }],
            response: Response {
                r#type: "Deletion confirmation".to_string(),
            },
        },
        Endpoint {
            path: "send_an_email_related_to_order".to_string(),
            description: "Send an email related to an order".to_string(),
            parameters: vec![Parameter {
                name: "email".to_string(),
                r#type: "string".to_string(),
                description: Some("Email address to send the notification to".to_string()),
                required: true,
            }],
            response: Response {
                r#type: "Email sent confirmation".to_string(),
            },
        },
    ]
}
