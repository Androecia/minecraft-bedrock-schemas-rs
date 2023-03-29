pub mod animation_controllers;
pub mod animations;
pub mod fog;

pub struct ResourcePack {
    pub animation_controllers: Vec<animation_controllers::AnimationControllers>,
    pub animations: Vec<animations::ActorAnimations>,
    pub fogs: Vec<fog::Fog>,
}
