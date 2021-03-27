#[derive(PartialEq, Debug, Clone)]
pub struct Icon(&'static str);

impl Icon {
    pub fn name(&self) -> &'static str {
        self.0
    }

    pub const HTTP: Icon = Icon("http");
    pub const DATABASE: Icon = Icon("database");
    pub const LOCK: Icon = Icon("lock");
    pub const INSERT: Icon = Icon("insert");
    pub const SELECT: Icon = Icon("select");
    pub const UPDATE: Icon = Icon("update");
    pub const DELETE: Icon = Icon("delete");
    pub const COMMIT: Icon = Icon("commit");
    pub const ROLLBACK: Icon = Icon("rollback");
    pub const SYSTEM: Icon = Icon("system");
    pub const DROP: Icon = Icon("drop");
    pub const CREATE: Icon = Icon("create");
    pub const ALTER: Icon = Icon("alter");
    pub const PLSQL: Icon = Icon("plsql");
    pub const LOGIN: Icon = Icon("login");
    pub const COPY: Icon = Icon("copy");
    pub const BOOKMARK: Icon = Icon("bookmark");
    pub const OTHER: Icon = Icon("other");
}
