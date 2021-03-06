use db::schema::*;

#[derive(Queryable, Serialize, AsChangeset)]
#[table_name = "category"]
#[primary_key(name)]
pub struct Category {
    pub name: String,
    pub abbr: String,
    pub description: String,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Serialize)]
#[table_name = "category_link"]
#[belongs_to(Category)]
#[primary_key(category_id, required_id)]
pub struct CategoryLink {
    pub category_id: String,
    pub required_id: String,
    pub level: i32,
}

#[derive(Queryable, Serialize, AsChangeset)]
#[table_name = "component"]
#[primary_key(name)]
pub struct Component {
    pub name: String,
    pub description: String,
    pub unit: String,
    pub cost: f32,
    pub weight: f32,
    pub volume: f32,
}

#[derive(Identifiable, Queryable, Insertable, Serialize, AsChangeset, FromForm)]
#[table_name = "page"]
#[primary_key(name)]
pub struct Page {
    pub name: String,
    pub text: String,
}

use diesel::data_types::PgInterval;

#[derive(Identifiable, Queryable, PartialEq, AsChangeset)]
#[table_name = "spell"]
#[primary_key(name)]
pub struct Spell {
    pub name: String,
    pub description: String,
    pub range: f32,
    pub casting_time: PgInterval,
    pub duration: Option<PgInterval>,
}

use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::collections::HashMap;

fn interval_to_hash(i: PgInterval) -> HashMap<&'static str, i32> {
    let mut hm : HashMap<&'static str, i32> = HashMap::new();
    let mut months = i.months;
    let mut days = i.days;
    let mut mcs = i.microseconds;
    if months > 12 {
        hm.insert("years", months/12);
        months = months % 12;
    }
    if months > 0 {
        hm.insert("months", i.months);
    }
    if days > 0 {
        hm.insert("days", i.days);
    }
    if mcs > 3600000000 {
        hm.insert("hours", (mcs/3600000000) as i32);
        mcs = mcs % 3600000000;
    }
    if mcs > 60000000 {
        hm.insert("minutes", (mcs/60000000) as i32);
        mcs = mcs % 60000000;
    }
    if mcs > 1000000 {
        hm.insert("seconds", (mcs/1000000) as i32);
        mcs = mcs % 1000000;
    }
    if mcs > 1000 {
        hm.insert("milliseconds", (mcs/1000) as i32);
        mcs = mcs % 1000;
    }
    hm
}

impl Serialize for Spell {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let casting_time = interval_to_hash(self.casting_time);
        let duration = self.duration.map(interval_to_hash);
        // TODO: format time better
        
        // 4 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Spell", 5)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("description", &self.description)?;
        state.serialize_field("range", &self.range)?;
        state.serialize_field("casting_time", &casting_time)?;
        state.serialize_field("duration", &duration)?;
        state.end()
    }
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Serialize)]
#[table_name = "spell_category"]
#[belongs_to(Spell)]
#[primary_key(spell_id, category_id)]
pub struct SpellCategory {
    pub spell_id: String,
    pub category_id: String,
    pub level: i32,
}

//use bigdecimal::BigDecimal;
#[derive(Identifiable, Queryable, Associations, PartialEq, Serialize)]
#[table_name = "spell_component"]
#[belongs_to(Spell)]
#[primary_key(spell_id, component_id)]
pub struct SpellComponent {
    pub spell_id: String,
    pub component_id: String,
    pub notes: String,
    //pub quantity: Option<BigDecimal>,
}
