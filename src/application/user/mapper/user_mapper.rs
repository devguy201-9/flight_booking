use crate::application::user::view::user_view::UserView;
use crate::domain::user::entity::User;

impl From<User> for UserView {
    fn from(u: User) -> Self {
        Self {
            id: u.id,
            avatar: u.avatar,
            first_name: u.first_name,
            last_name: u.last_name,
            username: u.username,
            email: u.email,
            birth_of_date: u.birth_of_date,
            display_name: u.display_name,
            gender: u.gender,
            phone_number: u.phone_number,
        }
    }
}
