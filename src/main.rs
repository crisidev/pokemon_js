/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

// This program is exported as a binary named `pokemon_service`.
use std::{net::SocketAddr, sync::Arc};

use aws_smithy_http_server::{AddExtensionLayer, Router};
use http::Extensions;
use neon::prelude::*;
use pokemon_service_sdk_js::{operation_registry::OperationRegistryBuilder, input::*, output::*, error::*};
use tower::ServiceBuilder;

pub async fn get_pokemon_species(input: GetPokemonSpeciesInput) -> Result<GetPokemonSpeciesOutput, GetPokemonSpeciesError> {
    todo!()
}

pub async fn get_server_statistics(input: GetServerStatisticsInput, ctx: Extensions<State>) -> GetServerStatisticsOutput {
    todo!()
}

pub async fn empty_operation(input: EmptyOperationInput) -> EmptyOperationOutput {
    todo!()
}

struct State {
    cx: FunctionContext<'static>
}

App().run()

pub fn asd(cx: ModuleContext) -> NeonResult<()> {
    let app: Router = OperationRegistryBuilder::default()
        // Build a registry containing implementations to all the operations in the service. These
        // are async functions or async closures that take as input the operation's input and
        // return the operation's output.
        .get_pokemon_species(get_pokemon_species)
        .get_server_statistics(get_server_statistics)
        .empty_operation(empty_operation)
        .build()
        .expect("Unable to build operation registry")
        // Convert it into a router that will route requests to the matching operation
        // implementation.
        .into();

    // Setup shared state and middlewares.
    // let shared_state = Arc::new(State::default());
    // let app = app.layer(
    //     ServiceBuilder::new()
    //         .layer(AddExtensionLayer::new(shared_state)),
    // );

    // Start the [`hyper::Server`].
    let bind: SocketAddr = "127.0.0.1:13734"
        .parse()
        .expect("unable to parse the server bind address and port");
    let server = hyper::Server::bind(&bind).serve(app.into_make_service());

    // // Run forever-ish...
    // if let Err(err) = server.await {
    //     eprintln!("server error: {}", err);
    // }
    Ok(())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    Ok(())
}
