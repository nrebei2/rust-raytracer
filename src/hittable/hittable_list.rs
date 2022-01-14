use std::rc::Rc;

use crate::hittable::{self, HitRecord, Hittable};

type Object = Rc<dyn hittable::Hittable>;

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

    pub fn add(&mut self, object: Object) {
        self.objects.push(object)
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
        t_max: f64,
        rec: &mut hittable::HitRecord,
    ) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            // LOOK INTO
            let mut temp_rec = HitRecord::new();
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }
}
