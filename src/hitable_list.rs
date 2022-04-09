use crate::{
    ray::Ray,
    hitable::{HitRecord, Hitable}
};

#[derive(Clone)]
pub struct HitableList<T: Hitable> {
    pub list: Vec<T>,
}

impl<T: Hitable> HitableList<T> {
    pub fn new(list: Vec<T>) -> Self {
        HitableList { list }
    }
}

impl<T: Hitable> Hitable for HitableList<T> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for i in 0..self.list.len() {
            if self.list[i].hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }
}
