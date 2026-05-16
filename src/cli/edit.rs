use anyhow::{ Result, anyhow};
use clap::Parser;

use crate::{app::{App, Id}};

#[derive(Parser)]
pub struct Args {
    /// Existing Todo Id
    #[clap(short, long)]
    pub id: Id,
    #[clap(short, long)]
    pub title: String,
    #[clap(short, long)]
    pub description: Option<String>,
}

pub fn run (mut app:App, args:Args, ) -> Result<()>{
    let Args {
        id,
        title,
        description
    } = args;

    app.update_todo(id, title, description).ok_or_else(|| anyhow!("Todo with id {} not found", id))?;

    Ok(())
}
