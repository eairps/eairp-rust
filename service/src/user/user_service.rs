use domain::entity::user;
use domain::vo::user::user_info_vo::UserInfoVO;
use domain::dto::user::account_login_dto::AccountLoginDTO;
use utils::response::Response;

pub struct UserService;

impl UserService {
    pub fn new() -> Self {
        UserService
    }

    // pub fn user_login(&self, accountLoginDto: AccountLoginDTO) -> Result<Response<UserInfoVO>,  Box<dyn std::error::Error>> {
    //
    // }
}