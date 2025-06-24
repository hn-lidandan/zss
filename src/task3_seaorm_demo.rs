use dotenv::dotenv;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Database, DatabaseConnection, DeleteResult, EntityTrait,
    IntoActiveModel, QueryFilter,
};
use std::env;
mod entity;
use entity::student::{self, Entity as Student};
use entity::student::*;  // 导入所有导出项

async fn connect_db() -> Result<DatabaseConnection, sea_orm::DbErr> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    Database::connect(&db_url).await
}

// async fn create_student(
//     db: &DatabaseConnection,
//     name: String,
//     age: i32,
//     student_type: StudentType,
// ) -> Result<Student, sea_orm::DbErr> {

//     let new_student = student::ActiveModel{
//     name: sea_orm::ActiveValue::Set(name),
//     age: sea_orm::ActiveValue::Set(age),
//     student_type: sea_orm::ActiveValue::Set(student_type),
//     ..Default::default()
//     };
//     new_student.insert(db).await
// }

#[tokio::main]
async fn main() -> Result<(), sea_orm::DbErr> {
    let db = connect_db().await?;

    println!("lianjiechenggong!");

    let student1 = student::ActiveModel {
        name: sea_orm::Set("lidandan".to_string()),
        age: sea_orm::Set(18),
        r#type: sea_orm::Set("xuesheng".to_string()),
        ..Default::default()
    };
    student1.insert(&db).await?;
    println!("新增成功！！!");

    //修改数据
    update_stu(&db, 1, "lifangfang".to_string()).await?;

    //删除数据
    deledt_stu(&db, 2).await?;

    Ok(())
}

async fn update_stu(
    db: &DatabaseConnection,
    id: i32,
    new_name: String,
) -> Result<(), sea_orm::DbErr> {
    let stu = Student::find_by_id(id).one(db).await?.unwrap();
    let mut stu: student::ActiveModel = stu.into();
    stu.name = sea_orm::Set(new_name);
    stu.update(db).await?;
    println!("修改成功！！!");
    Ok(())
}

async fn deledt_stu(db: &DatabaseConnection, id: i32) -> Result<(), sea_orm::DbErr> {
    Student::delete_by_id(id).exec(db).await?;
    println!("删除成功！！!");
    Ok(())
}
