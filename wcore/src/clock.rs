use instant::Instant;

pub trait Clock {
    fn set_time(&mut self, time: u32);
    fn get_time(&mut self) -> u32;

    fn is_paused(&self) -> bool;
    fn set_paused(&mut self, value: bool, time: u32);
    fn toggle_paused(&mut self, time: u32);

    fn set_length(&mut self, value: u32);
    fn get_length(&self) -> u32;
}

pub struct SyncClock {
    last_pause: Instant,
    last_time: u32,
    paused: bool,
    length: u32,
}

impl SyncClock {
    pub fn new() -> Self {
        return Self {
            last_pause: Instant::now(),
            last_time: 0,
            paused: true,
            length: 0,
        };
    }
}

impl Clock for SyncClock {
    fn set_time(&mut self, time: u32) {
        self.last_pause = Instant::now();
        self.last_time = time;
    }

    fn get_time(&mut self) -> u32 {
        if self.paused {
            return self.last_time;
        } else {
            let now = instant::Instant::now();
            let diff = now.duration_since(self.last_pause).as_millis() as u32;
            let time = diff + self.last_time;

            if time >= self.length {
                self.paused = true;
                self.last_time = self.length;
            }

            return time;
        }
    }

    fn is_paused(&self) -> bool { return self.paused; }
    fn set_paused(&mut self, value: bool, time: u32) {
        self.paused = value;

        self.last_pause = Instant::now();
        self.last_time = time;
     }

    fn toggle_paused(&mut self, time: u32) {
        self.paused = !self.paused;
        
        self.last_pause = Instant::now();
        self.last_time = time;
    }
    
    fn set_length(&mut self, value: u32) { self.length = value; }
    fn get_length(&self) -> u32 { return self.length; }
}