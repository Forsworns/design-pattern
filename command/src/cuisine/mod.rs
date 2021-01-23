mod guang_dong;
mod jiang_su;
mod shan_dong;
mod si_chuan;

pub use guang_dong::*;
pub use jiang_su::*;
pub use shan_dong::*;
pub use si_chuan::*;

pub trait CuisineTrait: CuisineClone {
    // it should have been a nice case for associated type with cook trait, e.g.:
    // type Cook: CookTrait;
    // sadly, we cannot set associated type ...
    // otherwise, we have to supply the type parameter for trait object,
    // that is Box<dyn trait<assciated=xxx>>, and we cannot handle this

    fn cook(&self);
}

pub trait CuisineClone {
    fn clone_box(&self) -> Box<dyn CuisineTrait>;
}

impl<T> CuisineClone for T
where
    T: 'static + CuisineTrait + Clone,
{
    fn clone_box(&self) -> Box<dyn CuisineTrait> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn CuisineTrait> {
    fn clone(&self) -> Box<dyn CuisineTrait> {
        self.clone_box()
    }
}
