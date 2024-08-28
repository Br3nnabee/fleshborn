#[derive(Component, Debug, Clone, Deserialize)]
pub struct Weight(pub f32);

#[derive(Component, Debug, Clone, Deserialize)]
pub struct Tags(pub SmallVec<[String; 4]>);
