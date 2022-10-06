use std::collections::HashMap;

use aws_sdk_dynamodb::{Client, model::AttributeValue, Error, client::fluent_builders::Query, output::{PutItemOutput, DeleteItemOutput, GetItemOutput}};
use serde::{Serialize, Deserialize};
use tokio_stream::StreamExt;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub usertype: String,
    pub password: String,
}

impl User {
    pub fn _new(id: String, username: String, password: String, usertype: String) -> Self {
        Self {
            id: id,
            username: username,
            password: password,
            usertype: usertype,
        }
    }
}

pub async fn add_item(client: &Client, user: User, table: &str) -> Result<PutItemOutput, Error> {
    let id = AttributeValue::N(user.id);
    let username = AttributeValue::S(user.username);
    let usertype = AttributeValue::N(user.usertype);
    let password = AttributeValue::S(user.password);

    let request = client
        .put_item()
        .table_name(table)
        .item("id", id)
        .item("username", username)
        .item("usertype", usertype)
        .item("password", password);

    println!("Executing request [{request:?}] to add item...");

    let resp = request.send().await?;
    Ok(resp)
}

pub async fn get_item(client: &Client, table: &str, key: &str, value: &str) -> Result<GetItemOutput, Error> {
    let found_user = client
    .get_item()
    .table_name(table)
    .key(key, AttributeValue::N(value.to_string()))
    .send()
    .await;

    match found_user {
        Ok(_) => Ok(found_user.ok().unwrap()),
        Err(e) => Err(Error::Unhandled(Box::new(e))),
    }
}

fn _get_by_type(client: &Client, table_name: &str, usertype: &u16) -> Query {
    client
        .query()
        .table_name(table_name)
        .key_condition_expression("usertype = :t")
        .expression_attribute_values(":t", AttributeValue::N(usertype.to_string()))
}

pub async fn list_items(client: &Client, table: &str) -> Result<Vec<HashMap<String, AttributeValue>>, Error> {
    let items: Result<Vec<_>, _> = client
        .scan()
        .table_name(table)
        .into_paginator()
        .items()
        .send()
        .collect()
        .await;

        
    match items {
        Ok(users) => Ok(users),
        Err(e) => Err(Error::Unhandled(Box::new(e))),
    }
}

pub async fn delete_item(client: &Client, table: &str, key: &str, value: &str) -> Result<DeleteItemOutput, Error> {
    match client
        .delete_item()
        .table_name(table)
        .key(key, AttributeValue::N(value.to_string()))
        .send()
        .await
    {
        Ok(res) => {
            println!("Deleted item from table");
            Ok(res)
        },
        Err(e) => Err(Error::Unhandled(Box::new(e))),
    }
}