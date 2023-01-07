use entity::{
    sea_orm_active_enums::{Role, Status},
    user,
};
use juniper::GraphQLObject;

#[derive(GraphQLObject, Debug)]
pub struct GQLUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: Role,
    pub status: Status,
}

impl GQLUser {
    pub fn single(model: &user::Model) -> Self {
        GQLUser {
            id: model.id.to_string(),
            name: model.name.to_string(),
            email: model.email.to_string(),
            role: model.role.to_owned(),
            status: model.status.to_owned(),
        }
    }

    pub fn multiple(models: Vec<user::Model>) -> Vec<Self> {
        models
            .into_iter()
            .map(|model| GQLUser::single(&model))
            .collect()
    }
    pub fn from_active_model(model: user::ActiveModel) -> Self {
        let user = user::Model {
            id: model.id.unwrap(),
            email: model.email.unwrap(),
            name: model.name.unwrap(),
            status: model.status.unwrap(),
            role: model.role.unwrap(),
            password: model.password.unwrap(),
        };
        GQLUser::single(&user)
    }
}
