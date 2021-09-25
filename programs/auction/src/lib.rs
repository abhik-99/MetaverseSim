use anchor_lang::prelude::*;
// use chrono::{DateTime, Utc};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod auction {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }

    pub fn add_board(ctx: Context<Addboard>, data: Board ) -> ProgramResult {
        Ok(())
    }

    pub fn start_auction(ctx: Context<StartAuction>, data: Auction) -> ProgramResult {
        Ok(())
    }

    pub fn place_bid(ctx: Context<PlaceBid>) -> ProgramResult {
        Ok(())
    }

    pub fn seal_bid(ctx: Context<SealBid>) -> ProgramResult {
        Ok(())
    }

    pub fn restore_board_defaults(ctx: Context<RestoreBoardDefaults>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct Addboard {}

#[derive(Accounts)]
pub struct StartAuction {}

#[derive(Accounts)]
pub struct PlaceBid {}

#[derive(Accounts)]
pub struct SealBid {}

#[derive(Accounts)]
pub struct RestoreBoardDefaults {}

#[account]
pub struct Auction {
    // Auction ID
    pub id: String,

    // Board which is being auctioned
    pub board: String,

    // If true then the auction is closed
    pub completed: bool,

    // Current Auction Iteration Start Date
    // (will be checked for proper matching with chrono::DateTime)
    pub start_date: String,

    // Current Auction Iteration End Date
    // (will be checked for proper matching with chrono::DateTime)
    pub end_date: String,

    // A Board can be auctioned/reauctioned upto 2^16-1 times
    pub iteration: u16
}

#[account]
pub struct Board {
    // Each Board is identified by an in-game ID
    pub id: String,

    // Image URL (initially it will the game poster)
    pub img_url: String,

    // Initially the game builders (us) are the owners
    pub owner: Pubkey,

    // This points to how many times the board has been auctioned
    pub auction_iteration: u16,

    // If true that would mean the Board is open to auction
    pub under_bid: bool,

    // Base Starting Price
    // Auction starts in every iteration from this base price
    pub base_price: f64,

    // Increments Allowed
    // Increments in price must be higher than this
    // for subsequent bids
    pub increments: f64,

    // Current Auction Price
    // If Auction for the board is completed
    // then shows the last highest price
    pub auction_price: f64
}
