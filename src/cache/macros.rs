// Bloom
//
// HTTP REST API caching middleware
// Copyright: 2017, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

macro_rules! get_cache_store_client {
    ($pool:expr, $error:expr, $client:ident $code:block) => {
        match $pool.get() {
            Ok($client) => $code,
            _ => Err($error),
        }
    };
}
