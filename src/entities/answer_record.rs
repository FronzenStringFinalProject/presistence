//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "answer_record"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq)]
pub struct Model {
    pub qid: i32,
    pub cid: i32,
    pub correct: bool,
    pub date: Date,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Qid,
    Cid,
    Correct,
    Date,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Cid,
    Qid,
    Date,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = (i32, i32, Date);
    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Children,
    Quizes,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Qid => ColumnType::Integer.def(),
            Self::Cid => ColumnType::Integer.def(),
            Self::Correct => ColumnType::Boolean.def(),
            Self::Date => ColumnType::Date.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Children => Entity::belongs_to(super::children::Entity)
                .from(Column::Cid)
                .to(super::children::Column::Cid)
                .into(),
            Self::Quizes => Entity::belongs_to(super::quizes::Entity)
                .from(Column::Qid)
                .to(super::quizes::Column::Qid)
                .into(),
        }
    }
}

impl Related<super::children::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Children.def()
    }
}

impl Related<super::quizes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Quizes.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
