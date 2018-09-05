use std::fmt;

pub enum ValueItem {
    NONE,
    ROE,
    PER,
    PBR,
}

pub struct Company {
    name : String,
    code : String,
    roe : f32,
    per : f32,
    pbr : f32,
}

// impl of Company
impl fmt::Debug for Company {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}, {}, {}, {}",
            self.name(), self.code(), self.roe(), self.per(), self.pbr)
    }
}

impl Company {
    pub fn new(name : &str, code : &str, roe : f32, per : f32, pbr : f32) -> Company {
        Company {
            name : name.to_string(),
            code : code.to_string(),
            roe : roe,
            per : per,
            pbr : pbr
        }
    }

    pub fn name(&self) -> &str { &self.name }
    pub fn code(&self) -> &str { &self.code }
    pub fn roe(&self) -> f32 { self.roe }
    pub fn per(&self) -> f32 { self.per }
    pub fn pbr(&self) -> f32 { self.pbr }
}