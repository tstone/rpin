use bevy::prelude::*;

use super::{AddCreditSwitch, StartButton, Switch};

/// A plugin that handles non-game machine operation:
/// - Encforcing payment
/// - Keeping track of how many paid players there are
#[derive(Debug, Clone)]
pub struct PaymentPlugin {
    /// 0 is free play
    pub required_credits: u8,
    pub max_credits: u8,
    pub max_players: u8,
}

impl Default for PaymentPlugin {
    fn default() -> Self {
        Self {
            required_credits: 0,
            max_credits: 20,
            max_players: 4,
        }
    }
}

impl Plugin for PaymentPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerPayments {
            credits_required: self.required_credits,
            max_credits: self.max_credits,
            max_players: self.max_players,
            ..Default::default()
        });

        app.add_systems(
            FixedUpdate,
            (accept_payment, add_player).run_if(in_state(PlayerOpenness::PlayersCanBeAdded)),
        );
    }
}

fn accept_payment(
    query: Query<&Switch, (With<AddCreditSwitch>, Changed<Switch>)>,
    mut payment: ResMut<PlayerPayments>,
    mut ev_credit_added: EventWriter<CreditAdded>,
    mut ev_max_credits: EventWriter<MaxCreditAdded>,
) {
    for switch in &query {
        if switch.closed {
            if payment.current_credits < payment.max_credits {
                payment.current_credits += 1;
                ev_credit_added.send(CreditAdded);
            } else {
                ev_max_credits.send(MaxCreditAdded);
            }
        }
    }
}

fn add_player(
    query: Query<&Switch, (With<StartButton>, Changed<Switch>)>,
    mut players: ResMut<PlayerPayments>,
    mut ev_player_added: EventWriter<PlayerAdded>,
) {
    // verify there are enough credits and we haven't reached max players
    if players.current_credits >= players.credits_required
        && players.paid_players < players.max_players
    {
        for button in &query {
            if button.closed {
                players.current_credits -= players.credits_required;
                players.paid_players += 1;
                ev_player_added.send(PlayerAdded);
            }
        }
    }
}

/// A state which defines if the machine is willing to accept additional players or not
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum PlayerOpenness {
    #[default]
    PlayersCanBeAdded,
    PlayersCannotBeAdded,
}

#[derive(Resource, Debug, Clone, PartialEq, Eq, Default)]
pub struct PlayerPayments {
    pub credits_required: u8,
    pub current_credits: u8,
    pub max_credits: u8,
    pub paid_players: u8,
    pub max_players: u8,
}

#[derive(Event)]
pub struct PlayerAdded;

#[derive(Event)]
pub struct CreditAdded;

#[derive(Event)]
pub struct MaxCreditAdded;
