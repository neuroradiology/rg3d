//! Contains all structures and methods to create and manage sprites.
//!
//! Sprite is billboard which always faces towards camera. It can be used
//! as a "model" for bullets, and so on.
//!
//! # Performance
//!
//! Huge amount of sprites may cause performance issues, also uou should
//! not use sprites to make particle systems, use ParticleSystem instead.

use crate::scene::node::Node;
use crate::{
    core::{
        color::Color,
        visitor::{Visit, VisitResult, Visitor},
    },
    resource::texture::Texture,
    scene::base::{Base, BaseBuilder},
};
use std::{
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex},
};

/// See module docs.
#[derive(Clone, Debug)]
pub struct Sprite {
    base: Base,
    texture: Option<Arc<Mutex<Texture>>>,
    color: Color,
    size: f32,
    rotation: f32,
}

impl Deref for Sprite {
    type Target = Base;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Sprite {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl Default for Sprite {
    fn default() -> Self {
        SpriteBuilder::new(BaseBuilder::new()).build()
    }
}

impl Sprite {
    /// Sets new size of sprite. Since sprite is always square, size
    /// defines half of width or height, so actual size will be doubled.    
    pub fn set_size(&mut self, size: f32) {
        self.size = size;
    }

    /// Returns current size of sprite.
    pub fn size(&self) -> f32 {
        self.size
    }

    /// Sets new color of sprite.
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    /// Returns current color of sprite.
    pub fn color(&self) -> Color {
        self.color
    }

    /// Sets rotation around "look" axis in radians.
    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    /// Returns rotation in radians.
    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    /// Sets new texture for sprite.
    pub fn set_texture(&mut self, texture: Arc<Mutex<Texture>>) {
        self.texture = Some(texture);
    }

    /// Returns current texture of sprite. Can be None if sprite has
    /// no texture.
    pub fn texture(&self) -> Option<Arc<Mutex<Texture>>> {
        self.texture.clone()
    }
}

impl Visit for Sprite {
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
        visitor.enter_region(name)?;

        self.texture.visit("Texture", visitor)?;
        self.color.visit("Color", visitor)?;
        self.size.visit("Size", visitor)?;
        self.rotation.visit("Rotation", visitor)?;
        self.base.visit("Base", visitor)?;

        visitor.leave_region()
    }
}

/// Sprite builder allows you to construct sprite in declarative manner.
/// This is typical implementation of Builder pattern.
pub struct SpriteBuilder {
    base_builder: BaseBuilder,
    texture: Option<Arc<Mutex<Texture>>>,
    color: Color,
    size: f32,
    rotation: f32,
}

impl SpriteBuilder {
    /// Creates new builder with default state (white opaque color, 0.2 size, zero rotation).
    pub fn new(base_builder: BaseBuilder) -> Self {
        Self {
            base_builder,
            texture: None,
            color: Color::WHITE,
            size: 0.2,
            rotation: 0.0,
        }
    }

    /// Sets desired texture.
    pub fn with_texture(mut self, texture: Arc<Mutex<Texture>>) -> Self {
        self.texture = Some(texture);
        self
    }

    /// Sets desired texture.
    pub fn with_opt_texture(mut self, texture: Option<Arc<Mutex<Texture>>>) -> Self {
        self.texture = texture;
        self
    }

    /// Sets desired color.
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Sets desired size.
    pub fn with_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Sets desired rotation.
    pub fn with_rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }

    /// Creates new sprite instance.
    pub fn build(self) -> Sprite {
        Sprite {
            base: self.base_builder.build(),
            texture: self.texture,
            color: self.color,
            size: self.size,
            rotation: self.rotation,
        }
    }

    /// Creates new node instance.
    pub fn build_node(self) -> Node {
        Node::Sprite(self.build())
    }
}
