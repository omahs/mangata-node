use crate::{
	Config, Pallet, ScheduleRewardsPerLiquidity, ScheduleRewardsTotal,
	TotalActivatedLiquidityForSchedules, SessionId
};
use core::marker::PhantomData;
use mangata_types::{Balance, TokenId};
use sp_core::U256;
use frame_support::pallet_prelude::*;


#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
/// Information about single token rewards. Automatically accumulates new rewards into `pending`
/// and once `pending_session_id < current_session` they are moved to `total` and become ready for
/// distribution to end users
pub struct ScheduleRewards {
    // Accumulated rewards in current or past session. Once `now > pending_session_id` they
    // should be moved to total
    pending: u128,

    // id of the session when pending_rewards were recently updated
    pending_session_id: SessionId,

    // total amount of rewards ready for distribution
    total: u128,
}


impl ScheduleRewards {
    pub fn provide_rewards(&mut self, now: SessionId, amount: u128) {
        if now <= self.pending_session_id {
            self.pending += amount;
        } else {
            self.total += self.pending;
            self.pending = amount;
            self.pending_session_id = now;
        }
    }

    pub fn total_rewards(&self, now: SessionId) -> u128 {
        if now <= self.pending_session_id {
            self.total
        } else {
            self.total + self.pending
        }
    }

    pub fn transfer_pending(&mut self, now: SessionId) {
        if now > self.pending_session_id {
            self.total += self.pending;
            self.pending = 0;
            self.pending_session_id = now;
        }
    }

    pub fn clear(&mut self, now: SessionId) {
        self.total = 0;
        self.pending = 0;
        self.pending_session_id = now;
    }
}


pub struct ScheduleRewardsCalculator<T> {
	data: PhantomData<T>,
}

/// Class responsible for maintaining and periodically updating cumulative
/// calculations required for 3rdparty rewards
impl<T: Config> ScheduleRewardsCalculator<T> {
	pub fn update_cumulative_rewards(
		liquidity_asset_id: TokenId,
		liquidity_assets_reward: TokenId,
	) {

        let session_id = Pallet::<T>::session_index() as u64;


		let (cumulative, idx) =
			ScheduleRewardsPerLiquidity::<T>::get((liquidity_asset_id, liquidity_assets_reward));
		if idx == (Pallet::<T>::session_index() as u64) {
		} else {
			let total_activated_liquidity =
				Self::total_activated_liquidity(liquidity_asset_id, liquidity_assets_reward);
			let total_schedule_rewards =
				Self::total_schedule_rewards(liquidity_asset_id, liquidity_assets_reward);
			if total_activated_liquidity > 0 {
				ScheduleRewardsTotal::<T>::mutate(
					(liquidity_asset_id, liquidity_assets_reward),
					|schedule| {
                        schedule.transfer_pending(session_id);
                        schedule.clear(session_id);
                    }
				);
				let pending = (U256::from(total_schedule_rewards) * U256::from(u128::MAX))
					.checked_div(U256::from(total_activated_liquidity))
					.unwrap_or_default();
				ScheduleRewardsPerLiquidity::<T>::insert(
					(liquidity_asset_id, liquidity_assets_reward),
					(cumulative + pending, (Pallet::<T>::session_index() as u64)),
				);
			}
		}
	}

	pub fn total_rewards_for_liquidity(
		liquidity_asset_id: TokenId,
		liquidity_assets_reward: TokenId,
	) -> U256 {
		let (cumulative, idx) =
			ScheduleRewardsPerLiquidity::<T>::get((liquidity_asset_id, liquidity_assets_reward));
		if idx == (Pallet::<T>::session_index() as u64) {
			cumulative
		} else {
			let total_activated_liquidity =
				Self::total_activated_liquidity(liquidity_asset_id, liquidity_assets_reward);
			let total_schedule_rewards =
				Self::total_schedule_rewards(liquidity_asset_id, liquidity_assets_reward);
			let pending = (U256::from(total_schedule_rewards) * U256::from(u128::MAX))
				.checked_div(U256::from(total_activated_liquidity))
				.unwrap_or_default();
			cumulative + pending
		}
	}

	pub fn total_activated_liquidity(
		liquidity_asset_id: TokenId,
		liquidity_assets_reward: TokenId,
	) -> Balance {
		let (pending_negative, pending_positive, idx, cumulative) =
			TotalActivatedLiquidityForSchedules::<T>::get(
				liquidity_asset_id,
				liquidity_assets_reward,
			);
		if idx == (Pallet::<T>::session_index() as u64) {
			cumulative
		} else {
			cumulative + pending_positive - pending_negative
		}
	}

	pub fn total_schedule_rewards(
		liquidity_asset_id: TokenId,
		liquidity_assets_reward: TokenId,
	) -> Balance {
        ScheduleRewardsTotal::<T>::get((liquidity_asset_id, liquidity_assets_reward)).total_rewards(Pallet::<T>::session_index() as u64)
	}


	pub fn update_total_activated_liqudity(
		liquidity_asset_id: TokenId,
		liquidity_assets_reward: TokenId,
		diff: Balance,
		change: bool,
	) {
		// TODO: make configurable
		let session_id = Pallet::<T>::session_index() as u64;

		TotalActivatedLiquidityForSchedules::<T>::mutate(
			liquidity_asset_id,
			liquidity_assets_reward,
			|(pending_negative, pending_positive, idx, cumulative)| {
				if *idx == session_id {
					if change {
						*pending_positive += diff;
					} else {
						*pending_negative += diff;
					};
				} else {
					// NOTE: handle burn so negative diff
					*cumulative = *cumulative + *pending_positive - *pending_negative;
					if change {
						*pending_positive = diff;
						*pending_negative = 0u128;
					} else {
						*pending_positive = 0u128;
						*pending_negative = diff;
					};
					*idx = session_id;
				}
			},
		);
	}
}