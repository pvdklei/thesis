pub struct Time {
    pub dt: f64,
    pub time: f64,
    pub gpu_timer: pgl::query::Query,
}

impl Time {
    pub fn new(starting_time: f64) -> Self {
        Self {
            time: starting_time,
            dt: std::f64::MIN,
            gpu_timer: pgl::query::Query::new(pgl::query::Target::TimeElapsed),
        }
    }
    pub fn update(&mut self, new_time: f64) {
        self.dt = new_time - self.time;
        self.time = new_time;
    }
}
