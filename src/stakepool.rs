use anyhow::anyhow;
use futures::FutureExt;

use subxt::{OnlineClient, PolkadotConfig};

use subxt::ext::codec::{Decode, Encode};
use subxt::tx::SubmittableExtrinsic;
use subxt::tx::TxPayload;
use subxt::utils::{AccountId32, MultiSignature};

use crate::services::{
    extension_signature_for_extrinsic, get_accounts, node_runtime,
    node_runtime::runtime_types::pallet_conviction_voting::vote::{AccountVote, Vote},
    subscribe_to_finalized_blocks, Account,
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub struct StakePoolComponent {

}