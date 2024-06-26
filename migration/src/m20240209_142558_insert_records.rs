use std::io::Cursor;

use chrono::NaiveDate;

use sea_orm_migration::{prelude::*, sea_orm::Set};
use serde::{
    de::{self, DeserializeOwned},
    Deserialize, Deserializer,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

const PARENT_CSV_STR: &str = include_str!("../../data/parent.csv");
const CHILD_CSV_STR: &str = include_str!("../../data/children.csv");
const ANS_RECORD_CSV_STR: &str = include_str!("../../data/ans_record.csv");
const QUIZ_CSV_STR: &str = include_str!("../../data/quiz.csv");
const QUIZ_LEVEL_CSV_STR: &str = include_str!("../../data/quiz_level.csv");

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        insert_from_csv::<QuizLevel>(QUIZ_LEVEL_CSV_STR, manager.get_connection()).await?;
        insert_from_csv::<Parent>(PARENT_CSV_STR, manager.get_connection()).await?;
        insert_from_csv::<Child>(CHILD_CSV_STR, manager.get_connection()).await?;
        insert_from_csv::<Quiz>(QUIZ_CSV_STR, manager.get_connection()).await?;
        insert_from_csv::<AnswerRecord>(ANS_RECORD_CSV_STR, manager.get_connection()).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        answer_record::Entity::delete_many()
            .exec(manager.get_connection())
            .await?;
        quizes::Entity::delete_many()
            .exec(manager.get_connection())
            .await?;
        children::Entity::delete_many()
            .exec(manager.get_connection())
            .await?;
        parent::Entity::delete_many()
            .exec(manager.get_connection())
            .await?;
        quiz_groups::Entity::delete_many()
            .exec(manager.get_connection())
            .await?;

        Ok(())
    }
}

fn deserialize_pg_bool<'de, D: Deserializer<'de>>(d: D) -> Result<bool, D::Error> {
    match char::deserialize(d)? {
        't' => Ok(true),
        'f' => Ok(false),
        _ => Err(de::Error::custom("expect 't' or 'f'")),
    }
}

trait IntoActive {
    type Active: ActiveModelTrait;

    fn into_active(self) -> Self::Active;
}

async fn insert_from_csv<'de, P>(csv: &'static str, db: &impl ConnectionTrait) -> Result<(), DbErr>
where
    P: DeserializeOwned + IntoActive,
{
    let mut rdr = csv::Reader::from_reader(Cursor::new(csv));
    let activates = rdr
        .deserialize::<P>()
        .map(|payload| payload.expect("CSV Error").into_active())
        .collect::<Vec<P::Active>>();

    let active_iter = activates.chunks(256);

    for active in active_iter {
        <P::Active as ActiveModelTrait>::Entity::insert_many(active.iter().cloned())
            .exec(db)
            .await?;
    }
    Ok(())
}

#[derive(Debug, Deserialize)]
struct AnswerRecord {
    cid: i32,
    qid: i32,
    #[serde(deserialize_with = "deserialize_pg_bool")]
    correct: bool,
    record_time: NaiveDate,
}

use presistence::{
    entities::{answer_record, children, parent, quiz_groups, quizes},
    sea_orm::{ActiveModelTrait, EntityTrait},
};

impl IntoActive for AnswerRecord {
    type Active = answer_record::ActiveModel;

    fn into_active(self) -> answer_record::ActiveModel {
        let AnswerRecord {
            cid,
            qid,
            correct,
            record_time,
        } = self;
        answer_record::ActiveModel {
            cid: Set(cid + 1),
            qid: Set(qid - 390),
            correct: Set(correct),
            date: Set(record_time),
            ..Default::default()
        }
    }
}

#[derive(Debug, Deserialize)]
struct Child {
    name: String,
    parent_id: i32,
    ability: f64,
}

impl IntoActive for Child {
    type Active = children::ActiveModel;

    fn into_active(self) -> children::ActiveModel {
        let Child {
            name,
            parent_id,
            ability,
            ..
        } = self;
        children::ActiveModel {
            name: Set(name),
            ability: Set(ability),
            parent: Set(parent_id),
            ..std::default::Default::default()
        }
    }
}

#[derive(Debug, Deserialize)]
struct Parent {
    name: String,
}

impl IntoActive for Parent {
    type Active = parent::ActiveModel;

    fn into_active(self) -> parent::ActiveModel {
        let Parent { name, .. } = self;
        parent::ActiveModel {
            name: Set(name),
            ..Default::default()
        }
    }
}

#[derive(Debug, Deserialize)]
struct QuizLevel {
    name: String,
}

impl IntoActive for QuizLevel {
    type Active = quiz_groups::ActiveModel;

    fn into_active(self) -> quiz_groups::ActiveModel {
        let QuizLevel { name, .. } = self;
        quiz_groups::ActiveModel {
            name: Set(name),
            ..Default::default()
        }
    }
}

#[derive(Debug, Deserialize)]
struct Quiz {
    quiz: String,
    ans: i32,
    level: i32,
    diff: f64,
    disc: f64,
    lambdas: f64,
}

impl IntoActive for Quiz {
    type Active = quizes::ActiveModel;

    fn into_active(self) -> quizes::ActiveModel {
        let Quiz {
            quiz,
            ans,
            level,
            diff,
            disc,
            lambdas,
            ..
        } = self;
        quizes::ActiveModel {
            quiz: Set(quiz),
            answer: Set(ans),
            group: Set(level),
            diff: Set(diff),
            disc: Set(disc),
            lambda: Set(lambdas),
            ..Default::default()
        }
    }
}
