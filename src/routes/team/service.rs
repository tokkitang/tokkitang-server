use std::{str::FromStr, sync::Arc};

use aws_sdk_dynamodb::{model::AttributeValue, Client};
use axum::Extension;
use std::error::Error;

use crate::{
    models::{Team, TeamUser},
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
            Err(error) => Err(AllError::AWSError(format!("{:?}", error))),
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
            Err(error) => Err(AllError::AWSError(format!("{:?}", error))),
        }
    }
}