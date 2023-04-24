use crate::object::Object;

struct World {
    objects: Vec<Box<dyn Object>>
}