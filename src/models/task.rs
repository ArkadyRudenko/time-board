use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use diesel::prelude::*;

pub struct Session(Duration, Duration);

impl Session {
    pub fn new() -> Self {
        let cur_time = SystemTime::now().get_current_time();
        Self {
            0: cur_time,
            1: cur_time,
        }
    }

    pub fn get_time(&self) -> Duration {
        self.1 - self.0
    }
}

pub struct Sessions(Vec<Session>);

#[derive(Queryable)]
pub struct Task {
    id: Uuid,
    description: String,
    sessions: Sessions,
}

// #[derive(Insertable)]
// #[diesel(table_name = tasks)]
// pub struct NewTask<'a> {
//     pub description: &'a str,
// }

trait CurrentTime {
    fn get_current_time(&self) -> Duration;
}

impl CurrentTime for SystemTime {
    fn get_current_time(&self) -> Duration {
        self.duration_since(UNIX_EPOCH).unwrap()
    }
}

impl Task {
    pub fn start_session(&mut self) {
        self.sessions.0.push(Session::new());
    }

    pub fn end_session(&mut self) {
        match self.sessions.0.last_mut() {
            Some(last) => {
                last.1 = SystemTime::now().get_current_time();
            }
            None => {}
        }
    }

    pub fn get_global_time(&self) -> Duration {
        let mut global_time = Duration::default();

        for session in &self.sessions.0 {
            global_time += session.get_time();
        }

        global_time
    }
}

