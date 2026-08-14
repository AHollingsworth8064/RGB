use std::any::TypeId;

struct Subscriber<T>{
    pub topic : String, 
    pub (in crate::nt) uid : u32,
    pub value : T, 
}

impl <T> Subscriber<T>  {
    pub fn  new(topic : String, value : T) -> Subscriber<T>{
        Subscriber { topic, uid: (1), value}
    }
}