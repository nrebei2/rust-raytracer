use std::rc::Rc;

use crate::hittable::{self, HitRecord, Hittable};

type Object = Box<dyn hittable::Hittable>;

pub struct HittableList {
    objects: Vec<Object>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }
    pub fn create(object: Object) -> Self {
        Self {
            objects: vec![object],
        }
    }

    pub fn add<M : Hittable + 'static>(&mut self, object: M) {
        self.objects.push(Box::new(object))
    }
    pub fn clear(&mut self) {
        self.objects.clear()
    }
}

impl Hittable for HittableList {
    fn hit(
        &self,
        r: &crate::Ray,
        t_min: f64,
        t_max: f64
    ) -> Option<HitRecord> {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        let mut rec: Option<HitRecord>  = None;

        for object in &self.objects {            
            if let Some(hit_rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = hit_rec.t;
                rec = Some (hit_rec);
            }
        }
        rec
    }
}
