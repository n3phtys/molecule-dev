use super::schema::*;

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



#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub screen_name: &'a str,
    pub original_site_id: i32,
}


#[derive(Queryable)]
pub struct User {
    pub user_id: i32,
    pub screen_name: String,
    pub first_name: String,
    pub last_name: String,
    pub anrede: String,
    pub geburtstag: i32,
    pub email: String,
    pub portrait: Option<i32>,
    pub original_site_id: i32,
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
