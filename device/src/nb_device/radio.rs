use super::TimestampMs;
pub use crate::radio::*;
pub use ::lora_modulation::{Bandwidth, CodingRate, SpreadingFactor};

#[derive(Debug)]
pub enum Event<'a, R>
where
    R: PhyRxTx,
{
    TxRequest(TxConfig, &'a [u8]),
    RxRequest(RfConfig),
    CancelRx,
    Phy(R::PhyEvent),
}

#[derive(Debug)]
pub enum Response<R>
where
    R: PhyRxTx,
{
    Idle,
    Txing,
    Rxing,
    TxDone(TimestampMs),
    RxDone(RxQuality),
    Phy(R::PhyResponse),
}

use core::fmt;

pub trait PhyRxTx {
    type PhyEvent: fmt::Debug;
    type PhyError: fmt::Debug;
    type PhyResponse: fmt::Debug;

    /// The antenna gain and board loss in dBi.
    const ANTENNA_GAIN: i8 = 0;

    /// The max power the radio can be instructed to transmit at. When preparing an instruction
    /// for the radio, this max power will be used an upper bound.
    const MAX_RADIO_POWER: u8;

    fn get_mut_radio(&mut self) -> &mut Self;

    // we require mutability so we may decrypt in place
    fn get_received_packet(&mut self) -> &mut [u8];
    fn handle_event(&mut self, event: Event<Self>) -> Result<Response<Self>, Self::PhyError>
    where
        Self: Sized;
}
