use blueprint_sdk::prelude::*;
use crate::context::MyContext;

pub const SQUARE_NUMBER_JOB_ID: u64 = 0;

#[debug_job]
pub async fn square_number(
    Context(_ctx): Context<MyContext>,
    TangleArgs1(number): TangleArgs1<i64>,
) -> Result<TangleResult<i64>> {
    let squared = number * number;
    
    // Log the input and result
    println!("Squaring number: {} -> {}", number, squared);
    
    // Return the squared result
    Ok(TangleResult::ok(squared))
}
