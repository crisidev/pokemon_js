#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(clippy::type_complexity)]
#![allow(rustdoc::bare_urls)]
//! The Pokémon Service allows you to retrieve information about Pokémon species.

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Errors that can occur when calling the service.
pub mod error;
/// Input structures for operations.
pub mod input;
mod json_ser;
/// Data structures used by operation inputs/outputs.
pub mod model;
/// All operations that this crate can perform.
pub mod operation;
mod operation_deser;
/// Operation handlers definition and implementation.
pub mod operation_handler;
/// A registry of your service's operations.
pub mod operation_registry;
mod operation_ser;
/// Output structures for operations.
pub mod output;
mod server_operation_handler_trait;
/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
/// Re-exported types from supporting crates.
pub mod types {
    pub use aws_smithy_http::byte_stream::AggregatedBytes;
    pub use aws_smithy_http::byte_stream::ByteStream;
    pub use aws_smithy_http::result::SdkError;
    pub use aws_smithy_types::Blob;
    pub use aws_smithy_types::DateTime;
}

use std::{
    any::Any,
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use crate::{error::*, input::*, operation_registry::OperationRegistryBuilder, output::*};
use aws_smithy_http_server::{AddExtensionLayer, Router};
use http::Extensions;
use neon::prelude::*;
use tower::ServiceBuilder;

pub async fn get_pokemon_species(
    input: GetPokemonSpeciesInput,
) -> Result<GetPokemonSpeciesOutput, GetPokemonSpeciesError> {
    todo!()
}

pub async fn get_server_statistics(input: GetServerStatisticsInput) -> GetServerStatisticsOutput {
    todo!()
}

pub async fn empty_operation(input: EmptyOperationInput) -> EmptyOperationOutput {
    todo!()
}

struct State<'a> {
    cx: &'a Arc<Mutex<ModuleContext<'a>>>,
}

struct App {}

impl App {
    pub fn run(mut cx: ModuleContext) -> NeonResult<()> {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .thread_name("smithy-rs-tokio")
            .build()
            .expect("Unable to start a new tokio runtime for this process");
        // Register operations into a Router, add middleware and start the `hyper` server,
        // all inside a [tokio] blocking function.
        rt.block_on(async move {
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
            // let v = Arc::new(Mutex::new(cx));
            // let shared_state = Arc::new(State { cx: &v });
            // let app = app.layer(
            //     ServiceBuilder::new()
            //         .layer(AddExtensionLayer::new(shared_state)),
            // );

            // Start the [`hyper::Server`].
            let bind: SocketAddr = "127.0.0.1:13734"
                .parse()
                .expect("unable to parse the server bind address and port");
            let server = hyper::Server::bind(&bind).serve(app.into_make_service());

            // Run forever-ish...
            if let Err(err) = server.await {
                eprintln!("server error: {}", err);
            }
        });
        Ok(())
    }
}

fn register_operations(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let mut handler_map = HashMap::new();
    let operations_handlers_handle: Handle<JsObject> = cx.argument(0)?;
    let operation_handlers =
        operations_handlers_handle.downcast_or_throw::<JsObject, _>(&mut cx)?;
    let property_names = operation_handlers.get_own_property_names(&mut cx)?;
    let v = property_names.to_vec(&mut cx)?;
    for value in v.iter() {
        let key = value.downcast::<JsString, _>(&mut cx).unwrap();
        let key_as_string: String = key.value(&mut cx);
        let obj = operation_handlers.get::<JsFunction, _, _>(&mut cx, key_as_string.as_str())?;
        handler_map.insert(key_as_string, Arc::new(obj));
    }
    println!("HM: {:#?}", handler_map);
    Ok(cx.undefined())
}

fn get_pokemon_species_input(mut cx: FunctionContext) -> JsResult<JsObject> {
    let args: Handle<JsString> = cx.argument(0)?;
    let name = args.value(&mut cx);
    let input = GetPokemonSpeciesInput::builder().name(name).build().unwrap();
    let obj = input.to_object(&mut cx)?;
    Ok(obj)
}

// create_server :: ( () -> Promise<HashMap<String, Function>> ) -> Undefined
fn create_server(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let operation_handlers_promise_handle: Handle<JsObject> = cx
        .argument::<JsFunction>(0)?
        .call_with(&mut cx)
        .this(cx.null())
        .apply::<JsObject, _>(&mut cx)?;
    operation_handlers_promise_handle
        .get::<JsFunction, _, _>(&mut cx, "then")?
        .call_with(&mut cx)
        .this(operation_handlers_promise_handle)
        .arg(JsFunction::new(&mut cx, register_operations)?)
        .apply::<JsPromise, _>(&mut cx)?;
    Ok(cx.undefined())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("createServer", create_server)?;
    cx.export_function("GetPokemonSpeciesInput", get_pokemon_species_input)?;
    Ok(())
}
