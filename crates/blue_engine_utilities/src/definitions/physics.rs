/*
 * Blue Engine by Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

#![cfg(feature = "physics")]

use blue_engine::{StringBuffer, glm};
use rapier3d::prelude::*;
use std::collections::HashMap;

/// Plugin for physics.
pub struct Physics {
    pub rigid_body_set: RigidBodySet,
    pub rigid_body_set_map: HashMap<String, RigidBodyHandle>,
    pub collider_set: ColliderSet,
    pub collider_set_map: HashMap<String, ColliderHandle>,
    pub gravity: glm::Vec3,
    pub integration_parameters: IntegrationParameters,
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub broad_phase: DefaultBroadPhase,
    pub narrow_phase: NarrowPhase,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
    pub physics_hooks: Box<dyn PhysicsHooks>,
    pub event_handler: Box<dyn EventHandler>,
    pub query_pipeline: QueryPipeline,
}
impl Physics {
    /// Creates a new physics plugin.
    pub fn new() -> Self {
        Self {
            rigid_body_set: RigidBodySet::new(),
            rigid_body_set_map: HashMap::new(),
            collider_set: ColliderSet::new(),
            collider_set_map: HashMap::new(),
            gravity: vector![0.0, -9.81, 0.0],
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            physics_hooks: Box::new(()),
            event_handler: Box::new(()),
            query_pipeline: QueryPipeline::new(),
        }
    }

    /// Inserts a collider into the physics world.
    /// Returns the handle of the collider.
    ///
    /// # Arguments
    /// * `name` - The name of the collider.
    /// * `collider` - The collider to insert
    pub fn insert_collider(
        &mut self,
        name: impl StringBuffer,
        collider: Collider,
    ) -> ColliderHandle {
        let handle = self.collider_set.insert(collider.clone());
        self.collider_set_map.insert(name.as_string(), handle);
        handle
    }

    /// Inserts a collider into the physics world and also sets the parent of the collider.
    /// Returns the handle of the collider.
    ///
    /// # Arguments
    /// * `name` - The name of the collider.
    /// * `collider` - The collider to insert
    /// * `parent` - The parent of the collider
    pub fn insert_collider_with_parent(
        &mut self,
        name: impl StringBuffer,
        collider: Collider,
        parent: RigidBodyHandle,
    ) -> ColliderHandle {
        let handle = self.collider_set.insert_with_parent(
            collider.clone(),
            parent,
            &mut self.rigid_body_set,
        );
        self.collider_set_map.insert(name.as_string(), handle);
        handle
    }

    /// Inserts a rigid body into the physics world.
    /// Returns the handle of the rigid body.
    ///
    /// # Arguments
    /// * `name` - The name of the rigid body.
    /// * `rigid_body` - The rigid body to insert
    pub fn insert_rigid_body(
        &mut self,
        name: impl StringBuffer,
        rigid_body: RigidBody,
    ) -> RigidBodyHandle {
        let handle = self.rigid_body_set.insert(rigid_body.clone());
        self.rigid_body_set_map.insert(name.as_string(), handle);
        handle
    }

    /// Removes a collider from the physics world.
    ///
    /// # Arguments
    /// * `name` - The name of the collider.
    /// * `collider` - The collider to remove.
    pub fn remove_collider(&mut self, name: impl StringBuffer, collider: ColliderHandle) {
        self.collider_set.remove(
            collider,
            &mut self.island_manager,
            &mut self.rigid_body_set,
            false,
        );
        self.collider_set_map.remove(&name.as_string());
    }

    /// Removes a rigid body from the physics world.
    ///
    /// # Arguments
    /// * `name` - The name of the rigid body.
    /// * `rigid_body` - The rigid body to remove.
    pub fn remove_rigid_body(&mut self, name: impl StringBuffer, rigid_body: RigidBodyHandle) {
        self.rigid_body_set.remove(
            rigid_body,
            &mut self.island_manager,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            false,
        );
        self.rigid_body_set_map.remove(&name.as_string());
    }
}

impl Default for Physics {
    fn default() -> Self {
        Self::new()
    }
}

impl blue_engine::Signal for Physics {
    fn frame(
        &mut self,
        _renderer: &mut blue_engine::Renderer,
        _window: &blue_engine::Window,
        objects: &mut blue_engine::ObjectStorage,
        _camera: &mut blue_engine::CameraContainer,
        _input: &blue_engine::InputHelper,
        _encoder: &mut blue_engine::CommandEncoder,
        _view: &blue_engine::TextureView,
    ) {
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            None,
            self.physics_hooks.as_ref(),
            self.event_handler.as_ref(),
        );
        self.query_pipeline.update(&self.collider_set);

        for i in self.rigid_body_set_map.iter() {
            let object = objects.get_mut(i.0);
            if object.is_some() {
                let position = self.rigid_body_set[*i.1].translation();

                object
                    .unwrap()
                    .set_position([position.x, position.y, position.z]);
            }
        }
    }
}
