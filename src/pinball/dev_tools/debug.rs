use std::{fmt::Debug, hash::Hash};

use bevy::{dev_tools::states::*, prelude::*};

use crate::pinball::{
    payment::{
        AddPlayerState, CreditAdded, MaxCreditAdded, PaymentState, PlayerAdded, PlayerPayments,
    },
    CabinetButtons, CabinetSwitches, LowerThirdsSwitches, SwitchInput,
};

/// A plugin which prints logs for pinball events
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PinballDebugLogger;

impl Plugin for PinballDebugLogger {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                log_input::<CabinetButtons>,
                log_input::<CabinetSwitches>,
                log_input::<LowerThirdsSwitches>,
                log_credit_added,
                log_max_credit_added,
                log_player_added,
            ),
        )
        .add_systems(Update, log_transitions::<PaymentState>)
        .add_systems(Update, log_transitions::<AddPlayerState>);
    }
}

fn log_input<T: Debug + Copy + Eq + Hash + Send + Sync + 'static>(
    mut ev: EventReader<SwitchInput<T>>,
) {
    for event in ev.read() {
        trace!("Switch {:?} changed state: {:?}", event.id, event.state);
    }
}

fn log_credit_added(mut ev: EventReader<CreditAdded>, players: Res<PlayerPayments>) {
    for _ in ev.read() {
        debug!(
            "Credit added! {} of {}",
            players.current_credits, players.credits_required
        );
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
        info!(
            "Player added. {} players, {} credits.",
            players.paid_players, players.current_credits
        );
    }
}
