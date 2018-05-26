use super::schema::posts;

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}


#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

pub struct User {

}

pub struct File {

}

pub struct Structure {

}

pub struct Template {

}

pub struct Article {

}

pub struct Site {

}

pub struct Page {

}

pub struct Portlet {

}
