//! This file contains a helper physics struct and a bunch of helper conversion methods.

#[cfg(feature = "debug")]
use ggez::{
    graphics::{self, DrawParam, Rect},
    nalgebra::Point2,
    Context, GameResult,
};

#[cfg(feature = "debug")]
use ggez_goodies::{camera::Camera, nalgebra_glm::Vec2};

use nphysics2d::{
    material,
    nalgebra::{Isometry2, Vector2},
    ncollide2d::{
        query::ContactManifold,
        shape::{Cuboid, ShapeHandle},
    },
    object::{
        self, BodyPartHandle, BodyStatus, ColliderDesc, DefaultBodyHandle, RigidBody, RigidBodyDesc,
    },
    world,
};

use nphysics2d::nalgebra as na;

type N = f32;

/// Enum that is made for each physics object's identity
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ObjectData {
    Ground,
    Player,
    Enemy,
    Bullet,
    Barrel,
}

/// Helper physics struct that makes live's easier while using nphysics2d physics engine with ggez.
pub struct Physics {
    mechanical_world: world::DefaultMechanicalWorld<N>,
    geometrical_world: world::DefaultGeometricalWorld<N>,
    body_set: object::DefaultBodySet<N>,
    collider_set: object::DefaultColliderSet<N>,
    joint_constraint_set: nphysics2d::joint::DefaultJointConstraintSet<N>,
    force_generator_set: nphysics2d::force_generator::DefaultForceGeneratorSet<N>,
}

impl Physics {
    /// The amount of gravity for the Y axis in the physics world.
    const GRAVITY: N = 50.8;

    /// Create a new physics struct object.
    pub fn new() -> Self {
        let geometrical_world = world::DefaultGeometricalWorld::new();
        let gravity = Self::GRAVITY;

        let mechanical_world = world::DefaultMechanicalWorld::new(Vector2::new(0.0, gravity));

        let body_set = object::DefaultBodySet::new();
        let collider_set = object::DefaultColliderSet::new();

        let joint_constraint_set = nphysics2d::joint::DefaultJointConstraintSet::new();
        let force_generator_set = nphysics2d::force_generator::DefaultForceGeneratorSet::new();

        Self {
            geometrical_world,
            mechanical_world,
            body_set,
            collider_set,
            joint_constraint_set,
            force_generator_set,
        }
    }

    /// Step the physics world.
    pub fn step(&mut self) {
        self.mechanical_world.step(
            &mut self.geometrical_world,
            &mut self.body_set,
            &mut self.collider_set,
            &mut self.joint_constraint_set,
            &mut self.force_generator_set,
        );
    }

    // Creates a new tile body.
    pub fn create_tile(
        &mut self,
        pos: na::Point2<f32>,
        width: u16,
        height: u16,
    ) -> DefaultBodyHandle {
        let width = width as f32;
        let height = height as f32;

        let ground = RigidBodyDesc::new()
            .position(point_to_isometry(pos))
            .status(BodyStatus::Static)
            .build();
        let ground_handle = self.body_set.insert(ground);

        let shape = ShapeHandle::new(Cuboid::new(Vector2::new(
            width / 2.0 - 0.01,
            height / 2.0 - 0.01,
        )));
        let collider = ColliderDesc::new(shape)
            .material(material::MaterialHandle::new(material::BasicMaterial::new(
                0.0, 0.0,
            )))
            .user_data(ObjectData::Ground)
            .build(BodyPartHandle(ground_handle, 0));

        self.collider_set.insert(collider);

        ground_handle
    }

    /// Create a new player body.
    pub fn create_player(
        &mut self,
        pos: na::Point2<f32>,
        width: u16,
        height: u16,
    ) -> DefaultBodyHandle {
        let width = width as f32;
        let height = height as f32;

        let player = RigidBodyDesc::new()
            .position(point_to_isometry(pos))
            .mass(10.0)
            .linear_damping(1.0)
            .status(BodyStatus::Dynamic)
            .build();
        let player_handle = self.body_set.insert(player);

        let shape = ShapeHandle::new(Cuboid::new(Vector2::new(
            width / 2.0 - 0.01,
            height / 2.0 - 0.01,
        )));
        let collider = ColliderDesc::new(shape)
            .material(material::MaterialHandle::new(material::BasicMaterial::new(
                0.0, 0.0,
            )))
            .user_data(ObjectData::Player)
            .build(BodyPartHandle(player_handle, 0));

        self.collider_set.insert(collider);

        player_handle
    }

    /// Create a new enemy body.
    pub fn create_enemy(
        &mut self,
        pos: na::Point2<f32>,
        width: u16,
        height: u16,
    ) -> DefaultBodyHandle {
        let width = width as f32;
        let height = height as f32;

        let enemy = RigidBodyDesc::new()
            .position(point_to_isometry(pos))
            .mass(10.0)
            .linear_damping(1.0)
            .status(BodyStatus::Dynamic)
            .build();
        let enemy_handle = self.body_set.insert(enemy);

        let shape = ShapeHandle::new(Cuboid::new(Vector2::new(
            width / 2.0 - 0.01,
            height / 2.0 - 0.01,
        )));
        let collider = ColliderDesc::new(shape)
            .material(material::MaterialHandle::new(material::BasicMaterial::new(
                0.0, 0.0,
            )))
            .user_data(ObjectData::Enemy)
            .build(BodyPartHandle(enemy_handle, 0));

        self.collider_set.insert(collider);

        enemy_handle
    }

    /// Create a new enemy body.
    pub fn create_barrel(
        &mut self,
        pos: na::Point2<f32>,
        width: u16,
        height: u16,
    ) -> DefaultBodyHandle {
        let width = width as f32;
        let height = height as f32;

        let barrel = RigidBodyDesc::new()
            .position(point_to_isometry(pos))
            .mass(10.0)
            .linear_damping(1.0)
            .status(BodyStatus::Dynamic)
            .build();
        let barrel_handle = self.body_set.insert(barrel);

        let shape = ShapeHandle::new(Cuboid::new(Vector2::new(
            width / 2.0 - 0.01,
            height / 2.0 - 0.01,
        )));
        let collider = ColliderDesc::new(shape)
            .material(material::MaterialHandle::new(material::BasicMaterial::new(
                0.0, 0.0,
            )))
            .user_data(ObjectData::Barrel)
            .build(BodyPartHandle(barrel_handle, 0));

        self.collider_set.insert(collider);

        barrel_handle
    }

    /// Create a new bullet. Can be any included in crate::components::bullet::PlayerWepon enum
    pub fn create_bullet(
        &mut self,
        pos: na::Point2<f32>,
        width: u16,
        height: u16,
    ) -> DefaultBodyHandle {
        let width = width as f32;
        let height = height as f32;

        let bullet = RigidBodyDesc::new()
            .position(point_to_isometry(pos))
            .mass(10.0)
            .linear_damping(1.0)
            .status(BodyStatus::Dynamic)
            .build();
        let bullet_handle = self.body_set.insert(bullet);

        let shape = ShapeHandle::new(Cuboid::new(Vector2::new(
            width / 2.0 - 0.01,
            height / 2.0 - 0.01,
        )));
        let collider = ColliderDesc::new(shape)
            .material(material::MaterialHandle::new(material::BasicMaterial::new(
                0.0, 0.0,
            )))
            .user_data(ObjectData::Bullet)
            .build(BodyPartHandle(bullet_handle, 0));

        self.collider_set.insert(collider);

        bullet_handle
    }

    /// Returns a immutable body from the handle provided by the above helper functions.
    pub fn get_rigid_body(&mut self, handle: DefaultBodyHandle) -> &RigidBody<f32> {
        let body = self.body_set.rigid_body(handle).expect("Body not found!");

        body
    }

    /// Returns a mutable body from the handle provided by the above helper functions.
    pub fn get_rigid_body_mut(&mut self, handle: DefaultBodyHandle) -> &mut RigidBody<f32> {
        let body = self
            .body_set
            .rigid_body_mut(handle)
            .expect("Body not found!");

        body
    }

    /// Simple helper function that allows you to see the colliders.
    /// To be able to show the colliders run Call of Ferris by `cargo run --features=["debug"]`
    #[cfg(feature = "debug")]
    pub fn draw_colliders(&self, ctx: &mut Context, camera: &Camera) -> GameResult {
        for (_, collider) in self.collider_set.iter() {
            let shape = collider.shape().aabb(collider.position());

            let rect = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::Stroke(graphics::StrokeOptions::DEFAULT),
                Rect::new(0.0, 0.0, shape.extents().x, shape.extents().y),
                [1.0, 1.0, 1.0, 1.0].into(),
            )?;

            let pos = camera.calculate_dest_point(Vec2::new(shape.mins.x, shape.mins.y));

            graphics::draw(
                ctx,
                &rect,
                DrawParam::default()
                    .dest(Point2::new(pos.x, pos.y))
                    .offset(Point2::new(0.5, 0.5)),
            )?;
        }
        Ok(())
    }

    /// Returns all of the collisions with the provided object
    pub fn collisions(
        &mut self,
        object: DefaultBodyHandle,
    ) -> Vec<(
        (ObjectData, ObjectData),
        DefaultBodyHandle,
        &ContactManifold<f32>,
    )> {
        self.geometrical_world
            .contacts_with(&self.collider_set, object, true)
            .into_iter()
            .flatten()
            .map(|(handle1, _, handle2, _, _, manifold)| {
                (self.get_user_data(handle1, handle2), handle2, manifold)
            })
            .collect()
    }

    /// Gets the user data of the 2 handles provided in the collisions function.
    fn get_user_data(
        &self,
        obj1: DefaultBodyHandle,
        obj2: DefaultBodyHandle,
    ) -> (ObjectData, ObjectData) {
        let col1 = self.collider_set.get(obj1).unwrap();
        let col2 = self.collider_set.get(obj2).unwrap();

        let data1 = *col1
            .user_data()
            .unwrap()
            .downcast_ref::<ObjectData>()
            .expect("Invalid types");
        let data2 = *col2
            .user_data()
            .unwrap()
            .downcast_ref::<ObjectData>()
            .expect("Invalid types");

        (data1, data2)
    }

    pub fn destroy_body(&mut self, handle: DefaultBodyHandle) {
        self.body_set.remove(handle);
        self.collider_set.remove(handle);
    }
}

/// Converts isometry to point
pub fn isometry_to_point<N: na::RealField + Copy + na::Scalar>(
    isometry: &Isometry2<N>,
) -> na::Point2<N> {
    isometry.translation.vector.into()
}

/// Converts a point to isometry
pub fn point_to_isometry<N: na::RealField + Copy + na::Scalar>(
    point: na::Point2<N>,
) -> Isometry2<N> {
    Isometry2::translation(point.x, point.y)
}
