use super::schema::posts;

// 用于查询
#[derive(Queryable, Debug)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub body: String,
    pub published: bool,
}

// 用于创建
#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

// 伪装成一张表，为了获取最新插入的自动自增的ID （MySQL特有）
table! {
    sequences(id) {
        id -> BigInt,
    }
}

// 用于获取id
#[derive(QueryableByName)]
#[table_name="sequences"]
pub struct Sequence {
    pub id: i64,
}