use number_squarer_lib::{
    context::MyContext,
    jobs::{square_number, SQUARE_NUMBER_JOB_ID},
};
use blueprint_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<(), sdk::Error> {
    // Load the blueprint environment
    let env = BlueprintEnvironment::load()?;

    // Set up the signer from keystore
    let signer = env.keystore().first_local::<SpSr25519>()?;
    let pair = env.keystore().get_secret::<SpSr25519>(&signer)?;
    let signer = TanglePairSigner::new(pair.0);

    // Create Tangle client and producer/consumer
    let client = env.tangle_client().await?;
    let producer = TangleProducer::finalized_blocks(client.rpc_client.clone()).await?;
    let consumer = TangleConsumer::new(client.rpc_client.clone(), signer);
    
    // Initialize context
    let context = MyContext::new(env.clone()).await?;

    // Build and run the blueprint
    BlueprintRunner::builder(TangleConfig::default(), env)
        .router(
            Router::new()
                .route(SQUARE_NUMBER_JOB_ID, square_number.layer(TangleLayer))
                .with_context(context)
        )
        .producer(producer)
        .consumer(consumer)
        .run()
        .await
}
