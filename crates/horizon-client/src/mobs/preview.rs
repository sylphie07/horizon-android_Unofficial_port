//! Shared player-model preview rendering for the launcher and inventory.
use super::{Anim, Brain, Mob, MobData, MobType, Species};
use horizon_core::geometry::Vec3;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{Arc, Mutex, OnceLock, RwLock},
};

static PLAYER_TYPES: Mutex<Vec<(PathBuf, Arc<MobType>)>> = Mutex::new(Vec::new());

/// The player model type for each engine-data root, loaded once.
pub fn player_type(root: &Path) -> crate::Result<Arc<MobType>> {
    let mut cache = PLAYER_TYPES.lock().unwrap_or_else(|e| e.into_inner());
    if let Some((_, kind)) = cache.iter().find(|(cached, _)| cached == root) {
        return Ok(kind.clone());
    }
    let kind = load_player_type(root)?;
    cache.push((root.to_owned(), kind.clone()));
    Ok(kind)
}

fn load_player_type(root: &Path) -> crate::Result<Arc<MobType>> {
    let models = super::EntityModelDocument::load(root).map_err(|error| {
        format!("could not read {}/entity-models.json: {error}", root.display())
    })?;
    let model = models
        .model("minecraft:player#main")
        .ok_or_else(|| format!("entity-models.json in {} has no minecraft:player#main entry", root.display()))?;
    Ok(Arc::new(MobType {
            id: "minecraft:player".into(),
            species: Species::Zombie,
            width: 0.6,
            height: 1.8,
            eye_height: 1.62,
            category: "misc".into(),
            max_health: 20.0,
            speed: 0.1,
            flying_speed: 0.02,
            attack_damage: 1.0,
            attack_knockback: 0.0,
            follow_range: 0.0,
            step_height: 0.6,
            armor: 0.0,
            armor_toughness: 0.0,
            knockback_resistance: 0.0,
            jump_strength: 0.42,
            attributes: Default::default(),
            gravity: 0.08,
            safe_fall_distance: 3.0,
            fall_damage_multiplier: 1.0,
            water_efficiency: 0.0,
            water_slowdown: 0.8,
            air_speed: 0.02,
            baby_scale: 0.5,
            head_rot_speed: 10.0,
            max_head_x_rot: 40.0,
            max_head_y_rot: 75.0,
            ambient_interval: 80,
            flags: Default::default(),
            model: Arc::new(model),
            layers: ["player_slim#main", "player#cape"]
                .into_iter()
                .filter_map(|key| {
                    let model = models.model(&format!("minecraft:{key}"))?;
                    Some((key.to_owned(), Arc::new(model)))
                })
                .collect(),
            texture: "entity/player/wide/steve".into(),
            brain: Brain::Passive,
            anim: Anim::Humanoid { zombie_arms: false },
            sounds: [
                "entity.player.hurt",
                "entity.player.hurt",
                "entity.player.death",
            ],
            step_sound: None,
            food: Arc::from([]),
            tempt: Arc::from([]),
        }))
}

/// A reusable player preview. It owns only model/animation data; each draw
/// clones the lightweight mob pose so pointer rotation cannot affect gameplay.
pub struct PlayerPreview {
    mob: Mob,
    data: MobData,
}

impl PlayerPreview {
    /// Load the same baked player model and animation definitions used by the
    /// game, preserving a useful error when engine data is not installed.
    pub fn load(root: &Path) -> crate::Result<Self> {
        let kind = player_type(root)?;
        let mob = Mob::new(0, kind, Vec3::ZERO, 180.0);
        let data = MobData {
            states: Vec::new(),
            placements: HashMap::new(),
            heightmaps: HashMap::new(),
            animations: MobData::load_animations(root),
            villager_hats: RwLock::new(super::villager_hats::VillagerHats::load(root)),
            biome_tags: HashMap::new(),
            entity_tags: HashMap::new(),
            pois: OnceLock::new(),
            species_placements: OnceLock::new(),
        };
        Ok(Self { mob, data })
    }

    /// Draw the player inside a clipped, depth-tested GUI rectangle.
    /// skin is an atlas texture name and the profile's Classic/Slim choice.
    pub fn draw(
        &self,
        batch: &mut crate::hud::HudBatch,
        atlas: &crate::hud::HudAtlas,
        rect: [f32; 4],
        rotation: [f32; 2],
        hidden_model_parts: u8,
        skin: Option<(&str, bool)>,
    ) {
        let [x0, y0, x1, y1] = rect;
        let (cx, cy) = ((x0 + x1) / 2.0, (y0 + y1) / 2.0);
        let yaw = rotation[0];
        let pitch = rotation[1].clamp(-35.0, 35.0);
        let mut mob = self.mob.clone();
        mob.skin = skin.map(|(name, _)| name.to_owned());
        mob.slim = skin.is_some_and(|(_, slim)| slim);
        mob.extra.hidden_model_parts = hidden_model_parts;
        mob.extra.model_scale = 0.0;
        mob.previous = mob.position;
        mob.body_yaw = 180.0 + yaw;
        mob.previous_body_yaw = mob.body_yaw;
        mob.head_yaw = mob.body_yaw;
        mob.previous_head_yaw = mob.head_yaw;
        mob.head_pitch = -pitch;
        mob.previous_head_pitch = mob.head_pitch;
        let camera = glam::DVec3::new(mob.position.x, mob.position.y, mob.position.z);
        let lookup = |name: &str| {
            atlas
                .sprites
                .get(name)
                .map(|r| [r.u0, r.v0, r.u1, r.v1])
        };
        let mut vertices = Vec::new();
        super::render::render_with(
            &mob,
            1.0,
            camera,
            &lookup,
            240 | 240 << 8,
            &self.data,
            &[],
            &mut vertices,
        );
        let height = if mob.extra.crouching { 1.5 } else { 1.8 };
        // Scale the mannequin with the available preview area instead of
        // keeping the old fixed 78-unit model on every screen size.
        let model_scale = ((x1 - x0) * 0.52).min((y1 - y0) * 0.72).clamp(64.0, 220.0);
        let gui = glam::Mat4::from_translation(glam::Vec3::new(cx, cy, 0.0))
            * glam::Mat4::from_scale(glam::Vec3::new(model_scale, model_scale, -model_scale))
            * glam::Mat4::from_translation(glam::Vec3::new(0.0, height / 2.0, 0.0))
            * glam::Mat4::from_rotation_z(std::f32::consts::PI)
            * glam::Mat4::from_rotation_x(pitch.to_radians());
        let quads = player_preview_quads(vertices, gui, None, &atlas.sprites);
        let start = batch.vertices.len();
        for (_, positions, uv, color) in quads {
            batch.quad_3d(positions, uv, color);
        }
        batch.depth_since(start, rect);
    }
}

/// The inventory-preview lighting (Lighting.Entry.ENTITY_IN_UI).
pub fn entity_in_ui_shade(normal: glam::Vec3) -> f32 {
    const LIGHT_0: glam::Vec3 = glam::Vec3::new(0.2, -1.0, 1.0);
    const LIGHT_1: glam::Vec3 = glam::Vec3::new(-0.2, -1.0, 0.0);
    let light = LIGHT_0.normalize().dot(normal).max(0.0)
        + LIGHT_1.normalize().dot(normal).max(0.0);
    (light * 0.6 + 0.4).min(1.0)
}

/// Transform and shade the player model's quads for the GUI's depth pipeline.
/// This is shared with the in-game inventory preview.
pub fn player_preview_quads(
    mut verts: Vec<crate::mesher::Vertex>,
    gui: glam::Mat4,
    fire: Option<(f32, f32)>,
    sprites: &HashMap<String, crate::hud::Rect>,
) -> Vec<(f32, [[f32; 3]; 4], [[f32; 2]; 4], u32)> {
    let flame_start = verts.len();
    if let Some((width, height)) = fire {
        crate::fire_render::flames(
            [0.0; 3],
            width,
            height,
            glam::DVec3::ZERO,
            0.0,
            sprites,
            0xff,
            &mut verts,
        );
    }
    let mut quads = Vec::new();
    for (index, q) in verts.as_chunks::<4>().0.iter().enumerate() {
        let p: [glam::Vec3; 4] =
            std::array::from_fn(|i| gui.transform_point3(glam::Vec3::from_array(q[i].pos)));
        let n = (p[1] - p[0]).cross(p[2] - p[0]);
        if n.z >= 0.0 {
            continue;
        }
        let n = n.normalize_or_zero();
        let shade = if index * 4 >= flame_start {
            1.0
        } else {
            entity_in_ui_shade(-n)
        };
        let c = q[0].color;
        let ch = |shift: u32| ((((c >> shift) & 255) as f32 * shade) as u32) << shift;
        let color = (c & 0xFF00_0000) | ch(0) | ch(8) | ch(16);
        let depth = p.iter().map(|v| v.z).sum::<f32>() / 4.0;
        quads.push((
            depth,
            p.map(|v| v.to_array()),
            std::array::from_fn(|i| q[i].uv),
            color,
        ));
    }
    quads.sort_by(|a, b| a.0.total_cmp(&b.0));
    quads
}
