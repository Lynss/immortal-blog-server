use actix_session::Session;
use actix_web::actix::Addr;
pub use common::utils::*;
use common::{
    configs::{PRIVILEGES_PREFIX_KEY, USER_INFO_PREFIX_KEY},
    ImmortalError,
};
use futures::Future;
use share::{
    structs::{Privileges, UserAndPrivilegesInfo, UserId, UserInfo},
    DBExecutor,
};

pub fn get_user_and_privileges_info(
    db: Addr<DBExecutor>,
    id: i32,
) -> impl Future<Item = UserAndPrivilegesInfo, Error = ImmortalError> {
    db.send(UserId { id })
        .map_err(ImmortalError::ignore)
        .flatten()
}

pub fn storage_user_and_privileges_info(
    user_info: UserInfo,
    privileges: Privileges,
    id: i32,
    session: Session,
) {
    //save info into session;
    let privileges_prefix_key = create_prefix_key(PRIVILEGES_PREFIX_KEY, id).as_str();
    let user_info_prefix_key = create_prefix_key(USER_INFO_PREFIX_KEY, id).as_str();
    session.set(PRIVILEGES_PREFIX_KEY, privileges);
    session.set(PRIVILEGES_PREFIX_KEY, user_info);
}
