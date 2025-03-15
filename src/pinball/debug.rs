use bevy::{dev_tools::states::*, prelude::*};

use super::{
    payment::{CreditAdded, MaxCreditAdded, PlayerAdded, PlayerOpenness, PlayerPayments},
    PinballButton, Switch,
};

/// A plugin which prints logs for pinball events
#[derive(Debug, Clone)]
pub struct DebugLogger;

impl Plugin for DebugLogger {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                log_switch_changed,
                log_button_changed,
                log_credit_added,
                log_max_credit_added,
                log_player_added,
            ),
        )
        .add_systems(Update, log_transitions::<PlayerOpenness>);
    }
}

fn log_switch_changed(query: Query<&Switch, Changed<Switch>>) {
    for switch in query.iter() {
        let state = if switch.closed { "closed" } else { "open" };
        debug!("Switch {} changed state: {state}", switch.id);
    }
}

fn log_button_changed(query: Query<&PinballButton, Changed<PinballButton>>) {
    for button in query.iter() {
        let state = if button.down { "down" } else { "up" };
        debug!("Button {} changed state: {state}", button.switch_id);
    }
}

fn log_credit_added(mut ev: EventReader<CreditAdded>, players: Res<PlayerPayments>) {
    for _ in ev.read() {
        debug!("Credit added! Current credits: {}", players.current_credits);
    }
}

fn log_max_credit_added(mut ev: EventReader<MaxCreditAdded>, players: Res<PlayerPayments>) {
    for _ in ev.read() {
        debug!(
            "Max creddits added! Current credits: {}",
            players.current_credits
        );
    }
}

fn log_player_added(mut ev: EventReader<PlayerAdded>, players: Res<PlayerPayments>) {
    for _ in ev.read() {
        info!("Player added. {} players.", players.paid_players);
    }
}
