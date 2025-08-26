use sea_orm::prelude::*;

#[derive(Debug, Clone, PartiaLEq, Default, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "Account")]
pub struct Model {
	#[sea_orm(primary_key)]
	id: i32,
	email: String,
	password: String
}

#[derive(Debug, Clone, EnumIter, DeriveRelation)]
pub impl Relations {}

impl ActiveModelBehavior for ActiveModel {}