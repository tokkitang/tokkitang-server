use std::{str::FromStr, sync::Arc};

use aws_sdk_dynamodb::{model::AttributeValue, Client};
use axum::Extension;
use std::error::Error;

use crate::{
    models::{Team, TeamInvite, TeamUser},
    utils::AllError,
};

pub struct TeamService {
    client: Extension<Arc<Client>>,
}

impl TeamService {
    pub fn new(client: Extension<Arc<Client>>) -> Self {
        Self { client }
    }

    pub async fn create_team(&self, team_data: Team) -> Result<String, AllError> {
        let input = team_data.to_hashmap();

        match self
            .client
            .put_item()
            .table_name(Team::NAME)
            .set_item(input)
            .send()
            .await
        {
            Ok(_) => Ok(team_data.id),
            Err(error) => Err(AllError::AWSError(format!("{error:?}"))),
        }
    }

    pub async fn delete_team(&self, team_id: impl Into<String>) -> Result<(), AllError> {
        match self
            .client
            .delete_item()
            .table_name(Team::NAME)
            .key("id", AttributeValue::S(team_id.into()))
            .send()
            .await
        {
            Ok(_) => Ok(()),
            Err(error) => Err(AllError::AWSError(format!("{error:?}"))),
        }
    }

    pub async fn create_team_user(&self, team_user: TeamUser) -> Result<(), AllError> {
        let input = team_user.to_hashmap();

        match self
            .client
            .put_item()
            .table_name(TeamUser::NAME)
            .set_item(input)
            .send()
            .await
        {
            Ok(_) => Ok(()),
            Err(error) => Err(AllError::AWSError(format!("{error:?}"))),
        }
    }

    pub async fn delete_team_user(
        &self,
        team_id: impl Into<String>,
        user_id: impl Into<String>,
    ) -> Result<(), AllError> {
        match self
            .client
            .delete_item()
            .table_name(TeamUser::NAME)
            .key("team_id", AttributeValue::S(team_id.into()))
            .key("user_id", AttributeValue::S(user_id.into()))
            .send()
            .await
        {
            Ok(_) => Ok(()),
            Err(error) => Err(AllError::AWSError(format!("{error:?}"))),
        }
    }

    pub async fn get_team_by_id(&self, team_id: impl Into<String>) -> Result<Team, AllError> {
        match self
            .client
            .scan()
            .table_name(Team::NAME)
            .filter_expression("id = :team_id")
            .expression_attribute_values(":team_id", AttributeValue::S(team_id.into()))
            .send()
            .await
        {
            Ok(data) => data
                .items()
                .and_then(|items| {
                    items
                        .first()
                        .and_then(|item| Team::from_hashmap(item.to_owned()))
                })
                .ok_or(AllError::NotFound),
            Err(error) => Err(AllError::AWSError(format!("{error:?}"))),
        }
    }

    pub async fn get_team_user_list_by_user_id(
        &self,
        user_id: impl Into<String>,
    ) -> Result<Vec<TeamUser>, AllError> {
        let mut list = vec![];
        let mut last_evaluated_key = None;

        let user_id = user_id.into();

        loop {
            match self
                .client
                .scan()
                .table_name(TeamUser::NAME)
                .filter_expression("user_id = :user_id")
                .expression_attribute_values(":user_id", AttributeValue::S(user_id.clone()))
                .set_exclusive_start_key(last_evaluated_key)
                .send()
                .await
            {
                Ok(data) => {
                    if let Some(items) = data.items() {
                        for item in items {
                            if let Some(team_user) = TeamUser::from_hashmap(item.to_owned()) {
                                list.push(team_user);
                            }
                        }
                    }

                    match data.last_evaluated_key() {
                        None => return Ok(list),
                        Some(key) => {
                            last_evaluated_key = Some(key.to_owned());
                            continue;
                        }
                    }
                }
                Err(error) => return Err(AllError::AWSError(format!("{error:?}"))),
            }
        }
    }

    pub async fn get_team_user_list_by_team_id(
        &self,
        team_id: impl Into<String>,
    ) -> Result<Vec<TeamUser>, AllError> {
        let mut list = vec![];
        let mut last_evaluated_key = None;

        let team_id = team_id.into();

        loop {
            match self
                .client
                .scan()
                .table_name(TeamUser::NAME)
                .filter_expression("team_id = :team_id")
                .expression_attribute_values(":team_id", AttributeValue::S(team_id.clone()))
                .set_exclusive_start_key(last_evaluated_key)
                .send()
                .await
            {
                Ok(data) => {
                    if let Some(items) = data.items() {
                        for item in items {
                            if let Some(team_user) = TeamUser::from_hashmap(item.to_owned()) {
                                list.push(team_user);
                            }
                        }
                    }

                    match data.last_evaluated_key() {
                        None => return Ok(list),
                        Some(key) => {
                            last_evaluated_key = Some(key.to_owned());
                            continue;
                        }
                    }
                }
                Err(error) => return Err(AllError::AWSError(format!("{error:?}"))),
            }
        }
    }

    pub async fn find_team_user_by_team_and_user_id(
        &self,
        team_id: impl Into<String>,
        user_id: impl Into<String>,
    ) -> Result<Option<TeamUser>, AllError> {
        match self
            .client
            .scan()
            .table_name(TeamUser::NAME)
            .filter_expression("team_id = :team_id AND user_id = :user_id")
            .expression_attribute_values(":team_id", AttributeValue::S(team_id.into()))
            .expression_attribute_values(":user_id", AttributeValue::S(user_id.into()))
            .send()
            .await
        {
            Ok(data) => {
                if let Some(items) = data.items() {
                    if items.is_empty() {
                        Ok(None)
                    } else {
                        Ok(TeamUser::from_hashmap(items.first().unwrap().to_owned()))
                    }
                } else {
                    Ok(None)
                }
            }
            Err(error) => Err(AllError::AWSError(format!("{error:?}"))),
        }
    }

    pub async fn create_team_invite(&self, data: TeamInvite) -> Result<String, AllError> {
        let input = data.to_hashmap();

        match self
            .client
            .put_item()
            .table_name(TeamInvite::NAME)
            .set_item(input)
            .send()
            .await
        {
            Ok(_) => Ok(data.code),
            Err(error) => Err(AllError::AWSError(format!("{error:?}"))),
        }
    }

    pub async fn get_team_invite_by_code(
        &self,
        code: impl Into<String>,
    ) -> Result<TeamInvite, AllError> {
        match self
            .client
            .scan()
            .table_name(TeamInvite::NAME)
            .filter_expression("code = :code")
            .expression_attribute_values(":code", AttributeValue::S(code.into()))
            .send()
            .await
        {
            Ok(data) => data
                .items()
                .and_then(|items| {
                    items
                        .first()
                        .and_then(|item| TeamInvite::from_hashmap(item.to_owned()))
                })
                .ok_or(AllError::NotFound),
            Err(error) => Err(AllError::AWSError(format!("{error:?}"))),
        }
    }

    pub async fn delete_team_invite_by_code(
        &self,
        code: impl Into<String>,
    ) -> Result<(), AllError> {
        match self
            .client
            .delete_item()
            .table_name(TeamInvite::NAME)
            .key("code", AttributeValue::S(code.into()))
            .send()
            .await
        {
            Ok(_data) => Ok(()),
            Err(error) => Err(AllError::AWSError(format!("{error:?}"))),
        }
    }
}
