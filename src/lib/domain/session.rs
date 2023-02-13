use uuid::Uuid;

pub const ONE_DAY_IN_NANO: usize = 86400000000000;
pub const ONE_MINUTES_IN_NANO: usize = 60_000_000_000;
// ten minutes
pub const JWT_VALID_UNTIL: i64 = 600;

#[derive(Debug, Clone)]
pub struct WebSession {
    user_id: Uuid,
    email: String,
    session_id: String,
    created_at: i64,
    // can be user choice
    valid_until: usize,
    // one min delete
    is_active: bool,
    device_id: String,
    device_type: String,
    app_name: String,
    ip_address: String,
    location: String,
    last_active: i64,
    // device id
    // device_id: Uuid,  or send from device itself
}

impl WebSession {
    pub fn new(
        user_id: Uuid,
        email: String,
        session_id: &str,
        valid_until: usize,
        device_id: String,
        device_type: String,
        app_name: String,
        ip_address: String,
        location: String,
        created_at: i64,
    ) -> Self {
        let session_id = session_id.trim().to_string();
        let email = email.trim().to_lowercase();
        let device_id = device_id.trim().to_string();
        let device_type = device_type.trim().to_string();
        let app_name = app_name.trim().to_string();
        let ip_address = ip_address.trim().to_string();
        let location = location.trim().to_string();

        let is_active = true;
        let last_active = created_at;

        Self {
            user_id,
            email,
            session_id,
            created_at,
            valid_until,
            is_active,
            device_id,
            device_type,
            app_name,
            ip_address,
            location,
            last_active,
        }
    }

    pub fn into_inner(
        self,
    ) -> (
        Uuid,
        String,
        String,
        i64,
        usize,
        bool,
        String,
        String,
        String,
        String,
        String,
        i64,
    ) {
        (
            self.user_id,
            self.email,
            self.session_id,
            self.created_at,
            self.valid_until,
            self.is_active,
            self.device_id,
            self.device_type,
            self.app_name,
            self.ip_address,
            self.location,
            self.last_active,
        )
    }
}
