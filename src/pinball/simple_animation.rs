use bevy::animation::{
    animation_curves::{AnimatableProperty, AnimationCompatibleCurve},
    graph::{AnimationGraphHandle, AnimationNodeIndex},
    AnimationPlayer, AnimationTarget, AnimationTargetId,
};
use bevy::prelude::*;

pub struct SimpleAnimation {
    pub player: AnimationPlayer,
    pub animation_index: AnimationNodeIndex,
    pub graph_handle: AnimationGraphHandle,
    pub target: AnimationTarget,
}

impl SimpleAnimation {
    pub fn new<P: AnimatableProperty + Clone, C: AnimationCompatibleCurve<P::Property>>(
        name: &Name,
        entity: Entity,
        curve: AnimatableCurve<P, C>,
        mut animation_graphs: ResMut<Assets<AnimationGraph>>,
        mut animation_clips: ResMut<Assets<AnimationClip>>,
    ) -> SimpleAnimation {
        let target_id = AnimationTargetId::from_name(name);
        let mut clip = AnimationClip::default();
        clip.add_curve_to_target(target_id, curve);

        // This doesn't work because it removes it after one run through
        // clip.add_event_fn(*t, |commands, entity, _, _| {
        //     commands.entity(entity).remove::<AnimationPlayer>();
        //     commands.entity(entity).remove::<AnimationGraphHandle>();
        //     commands.entity(entity).remove::<AnimationTarget>();
        // });

        let clip_handle = animation_clips.add(clip);
        let (graph, node_index) = AnimationGraph::from_clip(clip_handle);
        let graph = animation_graphs.add(graph);

        let player = AnimationPlayer::default();

        SimpleAnimation {
            player,
            animation_index: node_index.clone(),
            graph_handle: AnimationGraphHandle(graph),
            target: AnimationTarget {
                id: target_id,
                player: entity,
            },
        }
    }

    pub fn to_bundle(self) -> impl Bundle {
        (self.player, self.graph_handle, self.target)
    }
}
