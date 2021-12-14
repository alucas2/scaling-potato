use crate::utility::*;
use crate::bvh::Bvh;

// ------------------------------------------- Hittable -------------------------------------------

#[derive(Clone)]
pub enum Hittable {
    Sphere {center: Rvec3, radius: Real, material_id: Id},
    List(Vec<Hittable>),
    Bvh(Bvh),
}

pub struct Hit {
    /// Distance of the hit position to the ray origin
    pub t: Real,
    /// Hit position
    pub position: Rvec3,
    /// Normal at the hit position as a unit vector
    pub normal: Rvec3,
    /// Material at the hit position
    pub material_id: Id,
}

impl Hittable {
    pub fn hit(&self, ray: &Ray) -> Option<Hit> {
        match self {
            Self::Sphere {center, radius, material_id} => hit_sphere(center, *radius, *material_id, ray),
            Self::List(list) => hit_list(list, ray),
            Self::Bvh(bvh) => bvh.hit(ray),
        }
    }

    pub fn bounding_box(&self) -> AABB {
        match self {
            Self::Sphere {center, radius, ..} => bounding_box_sphere(center, *radius),
            Self::List(list) => bounding_box_list(list),
            Self::Bvh(_) => panic!("Do not take the bounding box of a Bvh. What are you trying to do?")
        }
    }
}

// ------------------------------------------- Hit implementations -------------------------------------------

fn hit_sphere(center: &Rvec3, radius: Real, material_id: Id, ray: &Ray) -> Option<Hit> {
    let to_center = ray.origin - center;
    let a = ray.direction.norm_squared();
    let half_b = ray.direction.dot(&to_center);
    let c = to_center.norm_squared() - radius*radius;
    let delta = half_b*half_b - a*c;
    if delta <= 0.0 {
        return None
    }
    
    let sqrt_delta = delta.sqrt();
    let mut t = (-half_b - sqrt_delta) / a; // Try the closer hit
    if t < ray.t_min || t > ray.t_max {
        t = (-half_b + sqrt_delta) / a; // Try the further hit
        if t < ray.t_min || t > ray.t_max {
            return None
        }
    }

    let position = ray.at(t);
    let normal = (position - center).normalize();
    Some(Hit {t, position, normal, material_id})
}

fn hit_list(list: &[Hittable], ray: &Ray) -> Option<Hit> {
    let mut hit = None;
    let mut ray = ray.clone();
    for x in list {
        if let Some(new_hit) = x.hit(&ray) {
            ray.t_max = new_hit.t;
            hit.replace(new_hit);
        }
    }
    hit
}

// ------------------------------------------- Bounding box implementation -------------------------------------------

fn bounding_box_sphere(center: &Rvec3, radius: Real) -> AABB {
    AABB {
        min: center - vector![radius, radius, radius],
        max: center + vector![radius, radius, radius],
    }
}

fn bounding_box_list(list: &[Hittable]) -> AABB {
    if list.is_empty() {
        return AABB::default();
    }
    list.iter().skip(1).fold(list[0].bounding_box(), |aabb, x| aabb.union(&x.bounding_box()))
}
