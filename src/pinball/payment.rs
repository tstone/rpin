use bevy::prelude::*;

use super::{CabinetButtons, CabinetSwitches, MachineState, SwitchInput, SwitchState};

/// PaymentPlugin - A plugin that handles the payment mechanics.
///
/// # Outputs
///
/// ## Resources
/// - `PlayerPayments` - Config around payment and how many players have been added
///
/// ## State: AddPlayerState
/// - `NotAcceptingPlayers`
/// - `AcceptingPlayers` - (default)
/// - `MaxPlayers` - Maximum players reached
///
/// ## State: PaymentState
/// - `InsufficientCredits` - (default) for non-freeplay
/// - `SufficientCredits` - (default) when in freeplay
///
/// ## Events
/// - `PlayerAdded` - Fired whenever a player is added
/// - `CreditAdded` - Fired whenever a credit is added
/// - `MaxCreditAdded` - Fired if the machine is at max credits but another credit was added
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

        app.add_event::<CreditAdded>();
        app.add_event::<MaxCreditAdded>();
        app.add_event::<PlayerAdded>();

        app.init_state::<AddPlayerState>();

        // if in freeplay mode start with sufficient credits
        if self.required_credits == 0 {
            app.insert_state(PaymentState::SufficientCredits);
        } else {
            app.insert_state(PaymentState::InsufficientCredits);
        }

        app.add_systems(
            Update,
            accept_payment.run_if(in_state(AddPlayerState::AcceptingPlayers).and(payment_required)),
        );

        app.add_systems(
            Update,
            add_player.run_if(
                in_state(AddPlayerState::AcceptingPlayers)
                    .and(in_state(PaymentState::SufficientCredits)),
            ),
        );

        app.add_systems(OnExit(MachineState::InGame), reset);
    }
}

fn payment_required(payment: Res<PlayerPayments>) -> bool {
    payment.credits_required > 0
}

fn accept_payment(
    mut payment: ResMut<PlayerPayments>,
    mut ev_cab_switch: EventReader<SwitchInput<CabinetSwitches>>,
    mut ev_credit_added: EventWriter<CreditAdded>,
    mut ev_max_credits: EventWriter<MaxCreditAdded>,
    mut payment_state: ResMut<NextState<PaymentState>>,
) {
    for ev in ev_cab_switch.read() {
        if ev.state == SwitchState::Closed && ev.id == CabinetSwitches::AddCoin {
            if payment.current_credits < payment.max_credits {
                payment.current_credits += 1;
                ev_credit_added.send(CreditAdded);
                if payment.current_credits >= payment.credits_required {
                    payment_state.set(PaymentState::SufficientCredits);
                }
            } else {
                ev_max_credits.send(MaxCreditAdded);
            }
        }
    }
}

fn add_player(
    mut ev_cab_switch: EventReader<SwitchInput<CabinetButtons>>,
    mut payment: ResMut<PlayerPayments>,
    mut ev_player_added: EventWriter<PlayerAdded>,
    mut payment_state: ResMut<NextState<PaymentState>>,
    mut player_state: ResMut<NextState<AddPlayerState>>,
) {
    for ev in ev_cab_switch.read() {
        if ev.id == CabinetButtons::StartButton && ev.state == SwitchState::Closed {
            payment.current_credits -= payment.credits_required;
            payment.paid_players += 1;
            ev_player_added.send(PlayerAdded);

            if payment.paid_players == payment.max_players {
                player_state.set(AddPlayerState::MaxPlayers);
            }

            if payment.credits_required > 0 {
                if payment.current_credits >= payment.credits_required {
                    payment_state.set(PaymentState::SufficientCredits);
                } else {
                    payment_state.set(PaymentState::InsufficientCredits);
                }
            }
        }
    }
}

fn reset(mut payment: ResMut<PlayerPayments>) {
    payment.paid_players = 0;
}

/// A state which indicates if there are or aren't sufficient credits
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum PaymentState {
    #[default]
    InsufficientCredits,
    SufficientCredits,
}

/// A state which indicates if there are or aren't sufficient credits
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AddPlayerState {
    NotAcceptingPlayers,
    #[default]
    AcceptingPlayers,
    MaxPlayers,
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
