use rust_skeleton::v1::modules::user::dto::{
    transform_create_user_dto, transform_update_user_dto, CreateUserDto, UpdateUserDto,
};

#[test]
fn create_dto_rejects_invalid_email() {
    let dto = CreateUserDto {
        name: "foo".into(),
        email: "not-email".into(),
    };
    assert!(transform_create_user_dto(dto).is_err());
}

#[test]
fn create_dto_rejects_empty_name() {
    let dto = CreateUserDto {
        name: String::new(),
        email: "foo@bar.com".into(),
    };
    assert!(transform_create_user_dto(dto).is_err());
}

#[test]
fn update_dto_accepts_valid_payload() {
    let dto = UpdateUserDto {
        name: "foo".into(),
        email: "foo@bar.com".into(),
    };
    assert!(transform_update_user_dto(dto).is_ok());
}

#[test]
fn update_dto_rejects_invalid_email() {
    let dto = UpdateUserDto {
        name: "foo".into(),
        email: "not-email".into(),
    };
    assert!(transform_update_user_dto(dto).is_err());
}
