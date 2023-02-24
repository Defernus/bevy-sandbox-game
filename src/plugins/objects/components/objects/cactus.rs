use crate::plugins::{
    loading::resources::{GameAssets, PhysicsObject},
    objects::components::GameWorldObjectTrait,
};
use bevy_reflect::{FromReflect, Reflect};

#[derive(Debug, Clone, Default, Reflect, FromReflect)]
pub struct CactusObject;

impl CactusObject {
    const ID: &str = "cactus";
}

impl GameWorldObjectTrait for CactusObject {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn take(&mut self) -> Box<dyn GameWorldObjectTrait> {
        Box::new(std::mem::take(self))
    }

    fn get_clone(&self) -> Box<dyn GameWorldObjectTrait> {
        Box::new(self.clone())
    }

    fn deserialize(
        &self,
        _data: &[u8],
    ) -> Result<
        Box<dyn GameWorldObjectTrait>,
        crate::plugins::objects::components::ObjectDeserializationError,
    > {
        Ok(Box::new(Self::default()))
    }

    fn get_model<'a>(&self, assets: &'a GameAssets) -> &'a PhysicsObject {
        &assets.cactus_object
    }
}
