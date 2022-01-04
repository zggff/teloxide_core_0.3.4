//! Wrappers altering functionality of a bot.
//!
//! Bot adaptors are very similar to the [`Iterator`] adaptors: they are bots
//! wrapping other bots to alter existing or add new functionality.
//!
//! E.g. [`AutoSend`] allows `await`ing requests directly, no need to use
//! `.send()`.
//!
//! [`Requester`]: crate::requests::Requester

/// [`AutoSend`] bot adaptor which allows sending a request without calling
/// [`send`].
///
/// [`AutoSend`]: auto_send::AutoSend
/// [`send`]: crate::requests::Request::send
#[cfg(feature = "auto_send")]
#[cfg_attr(
    all(any(docsrs, dep_docsrs), feature = "nightly"),
    doc(cfg(feature = "auto_send"))
)]
pub mod auto_send;

/// [`CacheMe`] bot adaptor which caches [`GetMe`] requests.
///
/// [`CacheMe`]: cache_me::CacheMe
/// [`GetMe`]: crate::payloads::GetMe
#[cfg(feature = "cache_me")]
#[cfg_attr(
    all(any(docsrs, dep_docsrs), feature = "nightly"),
    doc(cfg(feature = "cache_me"))
)]
pub mod cache_me;

/// [`Trace`] bot adaptor which traces requests.
///
/// [`Trace`]: trace::Trace
#[cfg(feature = "trace_adaptor")]
#[cfg_attr(
    all(any(docsrs, dep_docsrs), feature = "nightly"),
    doc(cfg(feature = "trace_adaptor"))
)]
pub mod trace;

/// [`ErasedRequester`] bot adaptor which allows to erase type of
/// [`Requester`].
///
/// [`ErasedRequester`]: erased::ErasedRequester
/// [`Requester`]: crate::requests::Requester
#[cfg(feature = "erased")]
#[cfg_attr(
    all(any(docsrs, dep_docsrs), feature = "nightly"),
    doc(cfg(feature = "erased"))
)]
pub mod erased;

/// [`Throttle`] bot adaptor which allows automatically throttle when hitting
/// API limits.
///
/// [`Throttle`]: throttle::Throttle
#[cfg(feature = "throttle")]
#[cfg_attr(
    all(any(docsrs, dep_docsrs), feature = "nightly"),
    doc(cfg(feature = "throttle"))
)]
pub mod throttle;

mod parse_mode;

#[cfg(feature = "auto_send")]
#[cfg_attr(
    all(any(docsrs, dep_docsrs), feature = "nightly"),
    doc(cfg(feature = "auto_send"))
)]
pub use auto_send::AutoSend;
#[cfg(feature = "cache_me")]
#[cfg_attr(
    all(any(docsrs, dep_docsrs), feature = "nightly"),
    doc(cfg(feature = "cache_me"))
)]
pub use cache_me::CacheMe;
#[cfg(feature = "erased")]
#[cfg_attr(
    all(any(docsrs, dep_docsrs), feature = "nightly"),
    doc(cfg(feature = "erased"))
)]
pub use erased::ErasedRequester;
#[cfg(feature = "throttle")]
#[cfg_attr(
    all(any(docsrs, dep_docsrs), feature = "nightly"),
    doc(cfg(feature = "throttle"))
)]
pub use throttle::Throttle;
#[cfg(feature = "trace_adaptor")]
#[cfg_attr(
    all(any(docsrs, dep_docsrs), feature = "nightly"),
    doc(cfg(feature = "trace_adaptor"))
)]
pub use trace::Trace;

pub use parse_mode::DefaultParseMode;
