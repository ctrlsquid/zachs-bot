use rand::Rng;

use crate::{Context, Error};

struct DiceRoll(u32, u32, Option<i32>);

impl DiceRoll {
    /// A function for calculating the result of a dice roll
    fn roll(&self) -> u32 {
        let mut rng = rand::thread_rng();
        let mut total = 0;
        // Create a history of rolls
        let mut rolls = Vec::new();
        let num_dice = self.dice_roll();
        let num_sides = self.dice_sides();
        // For every dice to roll, roll it and add it to the total
        for _ in 0..num_dice {
            // Roll a random number between 1 and the number of sides on the dice
            let roll = rng.gen_range(1..num_sides);
            // Add the roll to the total
            total += rng.gen_range(1..num_sides);
            // Add the roll to the history of rolls
            rolls.push(roll);
        }
        let modifier = self.2.unwrap_or(0);
        total + modifier as u32
    }
    /// A function for getting the number of dice to roll
    fn dice_roll(&self) -> u32 {
        self.0
    }
    /// A function for getting the number of sides on the dice
    fn dice_sides(&self) -> u32 {
        self.1
    }
    /// A function for getting the modifier
    fn modifier(&self) -> Option<i32> {
        self.2
    }
}

/// Parses the syntax of the dice rolls
/// It returns a tuple of the number of dice to roll, the number of sides on the dice, and the modifier
// Format is: 1d20+5, which means roll 1 20-sided dice and add 5 to the result
fn parse_dice_roll(roll: &str) -> DiceRoll {
    // Get the number before the d
    let dice_roll: u32 = roll.clone().split("d").collect::<Vec<&str>>()[0].parse().unwrap();
    // Get the number after the d and before the +
    let dice_sides: u32 = roll.clone().split("d").collect::<Vec<&str>>()[1].split("+").collect::<Vec<&str>>()[0].parse().unwrap();
    // Get the number after the +
    let modifier = roll.clone().split("+").collect::<Vec<&str>>();
    // The modifier is optional, so if it's not there, set it to 0
    let modifier = if modifier.len() > 1 {
        Some(modifier[1].parse().unwrap())
    } else {
        None
    };

    DiceRoll(dice_roll, dice_sides, modifier)
}

/// Poise command to roll dice
#[poise::command(slash_command, prefix_command)]
pub(crate) async fn roll(
    ctx: Context<'_>,
    #[description = "Dice roll syntax"] roll: String,
) -> Result<(), Error> {
    let roll = parse_dice_roll(&roll);
    let base_response = format!("Rolling {} dice with {} sides...", roll.dice_roll(), roll.dice_sides());
    let response = if roll.modifier().is_none() {
        base_response
    } else {
        format!("{} and then adding {} to modify roll...", base_response, roll.modifier().unwrap())
    };
    ctx.say(response).await?;
    let result = roll.roll();
    let response = format!("The result is {}", result);
    ctx.say(response).await?;
    Ok(())
}