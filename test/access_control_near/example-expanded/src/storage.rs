pub enum StorageKey {
    Roles,
    Records,
    AdminRole(String),
    RoleData(String),
}

impl StorageKey {
    pub fn to_string(&self) -> String {
        match self {
            StorageKey::Records => "records".to_string(),
            StorageKey::Roles => "rol".to_string(),
            StorageKey::AdminRole(adm) => format!("{}adm", adm),
            StorageKey::RoleData(data) => format!("{}data", data),
        }
    }

    pub fn into_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}
