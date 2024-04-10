#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendPaymentRequest {
    /// The identity pubkey of the payment recipient
    #[prost(bytes = "vec", tag = "1")]
    pub dest: ::prost::alloc::vec::Vec<u8>,
    ///
    /// Number of satoshis to send.
    ///
    /// The fields amt and amt_msat are mutually exclusive.
    #[prost(int64, tag = "2")]
    pub amt: i64,
    /// The hash to use within the payment's HTLC
    #[prost(bytes = "vec", tag = "3")]
    pub payment_hash: ::prost::alloc::vec::Vec<u8>,
    ///
    /// The CLTV delta from the current height that should be used to set the
    /// timelock for the final hop.
    #[prost(int32, tag = "4")]
    pub final_cltv_delta: i32,
    ///
    /// A bare-bones invoice for a payment within the Lightning Network.  With the
    /// details of the invoice, the sender has all the data necessary to send a
    /// payment to the recipient. The amount in the payment request may be zero. In
    /// that case it is required to set the amt field as well. If no payment request
    /// is specified, the following fields are required: dest, amt and payment_hash.
    #[prost(string, tag = "5")]
    pub payment_request: ::prost::alloc::string::String,
    ///
    /// An upper limit on the amount of time we should spend when attempting to
    /// fulfill the payment. This is expressed in seconds. If we cannot make a
    /// successful payment within this time frame, an error will be returned.
    /// This field must be non-zero.
    #[prost(int32, tag = "6")]
    pub timeout_seconds: i32,
    ///
    /// The maximum number of satoshis that will be paid as a fee of the payment.
    /// If this field is left to the default value of 0, only zero-fee routes will
    /// be considered. This usually means single hop routes connecting directly to
    /// the destination. To send the payment without a fee limit, use max int here.
    ///
    /// The fields fee_limit_sat and fee_limit_msat are mutually exclusive.
    #[prost(int64, tag = "7")]
    pub fee_limit_sat: i64,
    ///
    /// Deprecated, use outgoing_chan_ids. The channel id of the channel that must
    /// be taken to the first hop. If zero, any channel may be used (unless
    /// outgoing_chan_ids are set).
    #[deprecated]
    #[prost(uint64, tag = "8")]
    pub outgoing_chan_id: u64,
    ///
    /// An optional maximum total time lock for the route. This should not
    /// exceed lnd's `--max-cltv-expiry` setting. If zero, then the value of
    /// `--max-cltv-expiry` is enforced.
    #[prost(int32, tag = "9")]
    pub cltv_limit: i32,
    ///
    /// Optional route hints to reach the destination through private channels.
    #[prost(message, repeated, tag = "10")]
    pub route_hints: ::prost::alloc::vec::Vec<super::lnrpc::RouteHint>,
    ///
    /// An optional field that can be used to pass an arbitrary set of TLV records
    /// to a peer which understands the new records. This can be used to pass
    /// application specific data during the payment attempt. Record types are
    /// required to be in the custom range >= 65536. When using REST, the values
    /// must be encoded as base64.
    #[prost(map = "uint64, bytes", tag = "11")]
    pub dest_custom_records: ::std::collections::HashMap<u64, ::prost::alloc::vec::Vec<u8>>,
    ///
    /// Number of millisatoshis to send.
    ///
    /// The fields amt and amt_msat are mutually exclusive.
    #[prost(int64, tag = "12")]
    pub amt_msat: i64,
    ///
    /// The maximum number of millisatoshis that will be paid as a fee of the
    /// payment. If this field is left to the default value of 0, only zero-fee
    /// routes will be considered. This usually means single hop routes connecting
    /// directly to the destination. To send the payment without a fee limit, use
    /// max int here.
    ///
    /// The fields fee_limit_sat and fee_limit_msat are mutually exclusive.
    #[prost(int64, tag = "13")]
    pub fee_limit_msat: i64,
    ///
    /// The pubkey of the last hop of the route. If empty, any hop may be used.
    #[prost(bytes = "vec", tag = "14")]
    pub last_hop_pubkey: ::prost::alloc::vec::Vec<u8>,
    /// If set, circular payments to self are permitted.
    #[prost(bool, tag = "15")]
    pub allow_self_payment: bool,
    ///
    /// Features assumed to be supported by the final node. All transitive feature
    /// dependencies must also be set properly. For a given feature bit pair, either
    /// optional or remote may be set, but not both. If this field is nil or empty,
    /// the router will try to load destination features from the graph as a
    /// fallback.
    #[prost(enumeration = "super::lnrpc::FeatureBit", repeated, tag = "16")]
    pub dest_features: ::prost::alloc::vec::Vec<i32>,
    ///
    /// The maximum number of partial payments that may be use to complete the full
    /// amount.
    #[prost(uint32, tag = "17")]
    pub max_parts: u32,
    ///
    /// If set, only the final payment update is streamed back. Intermediate updates
    /// that show which htlcs are still in flight are suppressed.
    #[prost(bool, tag = "18")]
    pub no_inflight_updates: bool,
    ///
    /// The channel ids of the channels are allowed for the first hop. If empty,
    /// any channel may be used.
    #[prost(uint64, repeated, tag = "19")]
    pub outgoing_chan_ids: ::prost::alloc::vec::Vec<u64>,
    ///
    /// An optional payment addr to be included within the last hop of the route.
    /// This is also called payment secret in specifications (e.g. BOLT 11).
    #[prost(bytes = "vec", tag = "20")]
    pub payment_addr: ::prost::alloc::vec::Vec<u8>,
    ///
    /// The largest payment split that should be attempted when making a payment if
    /// splitting is necessary. Setting this value will effectively cause lnd to
    /// split more aggressively, vs only when it thinks it needs to. Note that this
    /// value is in milli-satoshis.
    #[prost(uint64, tag = "21")]
    pub max_shard_size_msat: u64,
    ///
    /// If set, an AMP-payment will be attempted.
    #[prost(bool, tag = "22")]
    pub amp: bool,
    ///
    /// The time preference for this payment. Set to -1 to optimize for fees
    /// only, to 1 to optimize for reliability only or a value inbetween for a mix.
    #[prost(double, tag = "23")]
    pub time_pref: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackPaymentRequest {
    /// The hash of the payment to look up.
    #[prost(bytes = "vec", tag = "1")]
    pub payment_hash: ::prost::alloc::vec::Vec<u8>,
    ///
    /// If set, only the final payment update is streamed back. Intermediate updates
    /// that show which htlcs are still in flight are suppressed.
    #[prost(bool, tag = "2")]
    pub no_inflight_updates: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackPaymentsRequest {
    ///
    /// If set, only the final payment updates are streamed back. Intermediate
    /// updates that show which htlcs are still in flight are suppressed.
    #[prost(bool, tag = "1")]
    pub no_inflight_updates: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteFeeRequest {
    ///
    /// The destination one wishes to obtain a routing fee quote to. If set, this
    /// parameter requires the amt_sat parameter also to be set. This parameter
    /// combination triggers a graph based routing fee estimation as opposed to a
    /// payment probe based estimate in case a payment request is provided. The
    /// graph based estimation is an algorithm that is executed on the in memory
    /// graph. Hence its runtime is significantly shorter than a payment probe
    /// estimation that sends out actual payments to the network.
    #[prost(bytes = "vec", tag = "1")]
    pub dest: ::prost::alloc::vec::Vec<u8>,
    ///
    /// The amount one wishes to send to the target destination. It is only to be
    /// used in combination with the dest parameter.
    #[prost(int64, tag = "2")]
    pub amt_sat: i64,
    ///
    /// A payment request of the target node that the route fee request is intended
    /// for. Its parameters are input to probe payments that estimate routing fees.
    /// The timeout parameter can be specified to set a maximum time on the probing
    /// attempt. Cannot be used in combination with dest and amt_sat.
    #[prost(string, tag = "3")]
    pub payment_request: ::prost::alloc::string::String,
    ///
    /// A user preference of how long a probe payment should maximally be allowed to
    /// take, denoted in seconds. The probing payment loop is aborted if this
    /// timeout is reached. Note that the probing process itself can take longer
    /// than the timeout if the HTLC becomes delayed or stuck. Canceling the context
    /// of this call will not cancel the payment loop, the duration is only
    /// controlled by the timeout parameter.
    #[prost(uint32, tag = "4")]
    pub timeout: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteFeeResponse {
    ///
    /// A lower bound of the estimated fee to the target destination within the
    /// network, expressed in milli-satoshis.
    #[prost(int64, tag = "1")]
    pub routing_fee_msat: i64,
    ///
    /// An estimate of the worst case time delay that can occur. Note that callers
    /// will still need to factor in the final CLTV delta of the last hop into this
    /// value.
    #[prost(int64, tag = "2")]
    pub time_lock_delay: i64,
    ///
    /// An indication whether a probing payment succeeded or whether and why it
    /// failed. FAILURE_REASON_NONE indicates success.
    #[prost(enumeration = "super::lnrpc::PaymentFailureReason", tag = "5")]
    pub failure_reason: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendToRouteRequest {
    /// The payment hash to use for the HTLC.
    #[prost(bytes = "vec", tag = "1")]
    pub payment_hash: ::prost::alloc::vec::Vec<u8>,
    /// Route that should be used to attempt to complete the payment.
    #[prost(message, optional, tag = "2")]
    pub route: ::core::option::Option<super::lnrpc::Route>,
    ///
    /// Whether the payment should be marked as failed when a temporary error is
    /// returned from the given route. Set it to true so the payment won't be
    /// failed unless a terminal error is occurred, such as payment timeout, no
    /// routes, incorrect payment details, or insufficient funds.
    #[prost(bool, tag = "3")]
    pub skip_temp_err: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendToRouteResponse {
    /// The preimage obtained by making the payment.
    #[prost(bytes = "vec", tag = "1")]
    pub preimage: ::prost::alloc::vec::Vec<u8>,
    /// The failure message in case the payment failed.
    #[prost(message, optional, tag = "2")]
    pub failure: ::core::option::Option<super::lnrpc::Failure>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetMissionControlRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetMissionControlResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMissionControlRequest {}
/// QueryMissionControlResponse contains mission control state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMissionControlResponse {
    /// Node pair-level mission control state.
    #[prost(message, repeated, tag = "2")]
    pub pairs: ::prost::alloc::vec::Vec<PairHistory>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct XImportMissionControlRequest {
    /// Node pair-level mission control state to be imported.
    #[prost(message, repeated, tag = "1")]
    pub pairs: ::prost::alloc::vec::Vec<PairHistory>,
    /// Whether to force override MC pair history. Note that even with force
    /// override the failure pair is imported before the success pair and both
    /// still clamp existing failure/success amounts.
    #[prost(bool, tag = "2")]
    pub force: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct XImportMissionControlResponse {}
/// PairHistory contains the mission control state for a particular node pair.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PairHistory {
    /// The source node pubkey of the pair.
    #[prost(bytes = "vec", tag = "1")]
    pub node_from: ::prost::alloc::vec::Vec<u8>,
    /// The destination node pubkey of the pair.
    #[prost(bytes = "vec", tag = "2")]
    pub node_to: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "7")]
    pub history: ::core::option::Option<PairData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PairData {
    /// Time of last failure.
    #[prost(int64, tag = "1")]
    pub fail_time: i64,
    ///
    /// Lowest amount that failed to forward rounded to whole sats. This may be
    /// set to zero if the failure is independent of amount.
    #[prost(int64, tag = "2")]
    pub fail_amt_sat: i64,
    ///
    /// Lowest amount that failed to forward in millisats. This may be
    /// set to zero if the failure is independent of amount.
    #[prost(int64, tag = "4")]
    pub fail_amt_msat: i64,
    /// Time of last success.
    #[prost(int64, tag = "5")]
    pub success_time: i64,
    /// Highest amount that we could successfully forward rounded to whole sats.
    #[prost(int64, tag = "6")]
    pub success_amt_sat: i64,
    /// Highest amount that we could successfully forward in millisats.
    #[prost(int64, tag = "7")]
    pub success_amt_msat: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMissionControlConfigRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMissionControlConfigResponse {
    ///
    /// Mission control's currently active config.
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<MissionControlConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMissionControlConfigRequest {
    ///
    /// The config to set for mission control. Note that all values *must* be set,
    /// because the full config will be applied.
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<MissionControlConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMissionControlConfigResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MissionControlConfig {
    ///
    /// Deprecated, use AprioriParameters. The amount of time mission control will
    /// take to restore a penalized node or channel back to 50% success probability,
    /// expressed in seconds. Setting this value to a higher value will penalize
    /// failures for longer, making mission control less likely to route through
    /// nodes and channels that we have previously recorded failures for.
    #[deprecated]
    #[prost(uint64, tag = "1")]
    pub half_life_seconds: u64,
    ///
    /// Deprecated, use AprioriParameters. The probability of success mission
    /// control should assign to hop in a route where it has no other information
    /// available. Higher values will make mission control more willing to try hops
    /// that we have no information about, lower values will discourage trying these
    /// hops.
    #[deprecated]
    #[prost(float, tag = "2")]
    pub hop_probability: f32,
    ///
    /// Deprecated, use AprioriParameters. The importance that mission control
    /// should place on historical results, expressed as a value in \[0;1\]. Setting
    /// this value to 1 will ignore all historical payments and just use the hop
    /// probability to assess the probability of success for each hop. A zero value
    /// ignores hop probability completely and relies entirely on historical
    /// results, unless none are available.
    #[deprecated]
    #[prost(float, tag = "3")]
    pub weight: f32,
    ///
    /// The maximum number of payment results that mission control will store.
    #[prost(uint32, tag = "4")]
    pub maximum_payment_results: u32,
    ///
    /// The minimum time that must have passed since the previously recorded failure
    /// before we raise the failure amount.
    #[prost(uint64, tag = "5")]
    pub minimum_failure_relax_interval: u64,
    ///
    /// ProbabilityModel defines which probability estimator should be used in
    /// pathfinding. Note that the bimodal estimator is experimental.
    #[prost(enumeration = "mission_control_config::ProbabilityModel", tag = "6")]
    pub model: i32,
    ///
    /// EstimatorConfig is populated dependent on the estimator type.
    #[prost(oneof = "mission_control_config::EstimatorConfig", tags = "7, 8")]
    pub estimator_config: ::core::option::Option<mission_control_config::EstimatorConfig>,
}
/// Nested message and enum types in `MissionControlConfig`.
pub mod mission_control_config {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ProbabilityModel {
        Apriori = 0,
        Bimodal = 1,
    }
    impl ProbabilityModel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ProbabilityModel::Apriori => "APRIORI",
                ProbabilityModel::Bimodal => "BIMODAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "APRIORI" => Some(Self::Apriori),
                "BIMODAL" => Some(Self::Bimodal),
                _ => None,
            }
        }
    }
    ///
    /// EstimatorConfig is populated dependent on the estimator type.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EstimatorConfig {
        #[prost(message, tag = "7")]
        Apriori(super::AprioriParameters),
        #[prost(message, tag = "8")]
        Bimodal(super::BimodalParameters),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BimodalParameters {
    ///
    /// NodeWeight defines how strongly other previous forwardings on channels of a
    /// router should be taken into account when computing a channel's probability
    /// to route. The allowed values are in the range \[0, 1\], where a value of 0
    /// means that only direct information about a channel is taken into account.
    #[prost(double, tag = "1")]
    pub node_weight: f64,
    ///
    /// ScaleMsat describes the scale over which channels statistically have some
    /// liquidity left. The value determines how quickly the bimodal distribution
    /// drops off from the edges of a channel. A larger value (compared to typical
    /// channel capacities) means that the drop off is slow and that channel
    /// balances are distributed more uniformly. A small value leads to the
    /// assumption of very unbalanced channels.
    #[prost(uint64, tag = "2")]
    pub scale_msat: u64,
    ///
    /// DecayTime describes the information decay of knowledge about previous
    /// successes and failures in channels. The smaller the decay time, the quicker
    /// we forget about past forwardings.
    #[prost(uint64, tag = "3")]
    pub decay_time: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AprioriParameters {
    ///
    /// The amount of time mission control will take to restore a penalized node
    /// or channel back to 50% success probability, expressed in seconds. Setting
    /// this value to a higher value will penalize failures for longer, making
    /// mission control less likely to route through nodes and channels that we
    /// have previously recorded failures for.
    #[prost(uint64, tag = "1")]
    pub half_life_seconds: u64,
    ///
    /// The probability of success mission control should assign to hop in a route
    /// where it has no other information available. Higher values will make mission
    /// control more willing to try hops that we have no information about, lower
    /// values will discourage trying these hops.
    #[prost(double, tag = "2")]
    pub hop_probability: f64,
    ///
    /// The importance that mission control should place on historical results,
    /// expressed as a value in \[0;1\]. Setting this value to 1 will ignore all
    /// historical payments and just use the hop probability to assess the
    /// probability of success for each hop. A zero value ignores hop probability
    /// completely and relies entirely on historical results, unless none are
    /// available.
    #[prost(double, tag = "3")]
    pub weight: f64,
    ///
    /// The fraction of a channel's capacity that we consider to have liquidity. For
    /// amounts that come close to or exceed the fraction, an additional penalty is
    /// applied. A value of 1.0 disables the capacity factor. Allowed values are in
    /// \[0.75, 1.0\].
    #[prost(double, tag = "4")]
    pub capacity_fraction: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryProbabilityRequest {
    /// The source node pubkey of the pair.
    #[prost(bytes = "vec", tag = "1")]
    pub from_node: ::prost::alloc::vec::Vec<u8>,
    /// The destination node pubkey of the pair.
    #[prost(bytes = "vec", tag = "2")]
    pub to_node: ::prost::alloc::vec::Vec<u8>,
    /// The amount for which to calculate a probability.
    #[prost(int64, tag = "3")]
    pub amt_msat: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryProbabilityResponse {
    /// The success probability for the requested pair.
    #[prost(double, tag = "1")]
    pub probability: f64,
    /// The historical data for the requested pair.
    #[prost(message, optional, tag = "2")]
    pub history: ::core::option::Option<PairData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuildRouteRequest {
    ///
    /// The amount to send expressed in msat. If set to zero, the minimum routable
    /// amount is used.
    #[prost(int64, tag = "1")]
    pub amt_msat: i64,
    ///
    /// CLTV delta from the current height that should be used for the timelock
    /// of the final hop
    #[prost(int32, tag = "2")]
    pub final_cltv_delta: i32,
    ///
    /// The channel id of the channel that must be taken to the first hop. If zero,
    /// any channel may be used.
    #[prost(uint64, tag = "3")]
    pub outgoing_chan_id: u64,
    ///
    /// A list of hops that defines the route. This does not include the source hop
    /// pubkey.
    #[prost(bytes = "vec", repeated, tag = "4")]
    pub hop_pubkeys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    ///
    /// An optional payment addr to be included within the last hop of the route.
    /// This is also called payment secret in specifications (e.g. BOLT 11).
    #[prost(bytes = "vec", tag = "5")]
    pub payment_addr: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuildRouteResponse {
    ///
    /// Fully specified route that can be used to execute the payment.
    #[prost(message, optional, tag = "1")]
    pub route: ::core::option::Option<super::lnrpc::Route>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeHtlcEventsRequest {}
///
/// HtlcEvent contains the htlc event that was processed. These are served on a
/// best-effort basis; events are not persisted, delivery is not guaranteed
/// (in the event of a crash in the switch, forward events may be lost) and
/// some events may be replayed upon restart. Events consumed from this package
/// should be de-duplicated by the htlc's unique combination of incoming and
/// outgoing channel id and htlc id. \[EXPERIMENTAL\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HtlcEvent {
    ///
    /// The short channel id that the incoming htlc arrived at our node on. This
    /// value is zero for sends.
    #[prost(uint64, tag = "1")]
    pub incoming_channel_id: u64,
    ///
    /// The short channel id that the outgoing htlc left our node on. This value
    /// is zero for receives.
    #[prost(uint64, tag = "2")]
    pub outgoing_channel_id: u64,
    ///
    /// Incoming id is the index of the incoming htlc in the incoming channel.
    /// This value is zero for sends.
    #[prost(uint64, tag = "3")]
    pub incoming_htlc_id: u64,
    ///
    /// Outgoing id is the index of the outgoing htlc in the outgoing channel.
    /// This value is zero for receives.
    #[prost(uint64, tag = "4")]
    pub outgoing_htlc_id: u64,
    ///
    /// The time in unix nanoseconds that the event occurred.
    #[prost(uint64, tag = "5")]
    pub timestamp_ns: u64,
    ///
    /// The event type indicates whether the htlc was part of a send, receive or
    /// forward.
    #[prost(enumeration = "htlc_event::EventType", tag = "6")]
    pub event_type: i32,
    #[prost(oneof = "htlc_event::Event", tags = "7, 8, 9, 10, 11, 12")]
    pub event: ::core::option::Option<htlc_event::Event>,
}
/// Nested message and enum types in `HtlcEvent`.
pub mod htlc_event {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EventType {
        Unknown = 0,
        Send = 1,
        Receive = 2,
        Forward = 3,
    }
    impl EventType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EventType::Unknown => "UNKNOWN",
                EventType::Send => "SEND",
                EventType::Receive => "RECEIVE",
                EventType::Forward => "FORWARD",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "SEND" => Some(Self::Send),
                "RECEIVE" => Some(Self::Receive),
                "FORWARD" => Some(Self::Forward),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag = "7")]
        ForwardEvent(super::ForwardEvent),
        #[prost(message, tag = "8")]
        ForwardFailEvent(super::ForwardFailEvent),
        #[prost(message, tag = "9")]
        SettleEvent(super::SettleEvent),
        #[prost(message, tag = "10")]
        LinkFailEvent(super::LinkFailEvent),
        #[prost(message, tag = "11")]
        SubscribedEvent(super::SubscribedEvent),
        #[prost(message, tag = "12")]
        FinalHtlcEvent(super::FinalHtlcEvent),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HtlcInfo {
    /// The timelock on the incoming htlc.
    #[prost(uint32, tag = "1")]
    pub incoming_timelock: u32,
    /// The timelock on the outgoing htlc.
    #[prost(uint32, tag = "2")]
    pub outgoing_timelock: u32,
    /// The amount of the incoming htlc.
    #[prost(uint64, tag = "3")]
    pub incoming_amt_msat: u64,
    /// The amount of the outgoing htlc.
    #[prost(uint64, tag = "4")]
    pub outgoing_amt_msat: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ForwardEvent {
    /// Info contains details about the htlc that was forwarded.
    #[prost(message, optional, tag = "1")]
    pub info: ::core::option::Option<HtlcInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ForwardFailEvent {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SettleEvent {
    /// The revealed preimage.
    #[prost(bytes = "vec", tag = "1")]
    pub preimage: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalHtlcEvent {
    #[prost(bool, tag = "1")]
    pub settled: bool,
    #[prost(bool, tag = "2")]
    pub offchain: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribedEvent {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinkFailEvent {
    /// Info contains details about the htlc that we failed.
    #[prost(message, optional, tag = "1")]
    pub info: ::core::option::Option<HtlcInfo>,
    /// FailureCode is the BOLT error code for the failure.
    #[prost(enumeration = "super::lnrpc::failure::FailureCode", tag = "2")]
    pub wire_failure: i32,
    ///
    /// FailureDetail provides additional information about the reason for the
    /// failure. This detail enriches the information provided by the wire message
    /// and may be 'no detail' if the wire message requires no additional metadata.
    #[prost(enumeration = "FailureDetail", tag = "3")]
    pub failure_detail: i32,
    /// A string representation of the link failure.
    #[prost(string, tag = "4")]
    pub failure_string: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentStatus {
    /// Current state the payment is in.
    #[prost(enumeration = "PaymentState", tag = "1")]
    pub state: i32,
    ///
    /// The pre-image of the payment when state is SUCCEEDED.
    #[prost(bytes = "vec", tag = "2")]
    pub preimage: ::prost::alloc::vec::Vec<u8>,
    ///
    /// The HTLCs made in attempt to settle the payment \[EXPERIMENTAL\].
    #[prost(message, repeated, tag = "4")]
    pub htlcs: ::prost::alloc::vec::Vec<super::lnrpc::HtlcAttempt>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CircuitKey {
    /// / The id of the channel that the is part of this circuit.
    #[prost(uint64, tag = "1")]
    pub chan_id: u64,
    /// / The index of the incoming htlc in the incoming channel.
    #[prost(uint64, tag = "2")]
    pub htlc_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ForwardHtlcInterceptRequest {
    ///
    /// The key of this forwarded htlc. It defines the incoming channel id and
    /// the index in this channel.
    #[prost(message, optional, tag = "1")]
    pub incoming_circuit_key: ::core::option::Option<CircuitKey>,
    /// The incoming htlc amount.
    #[prost(uint64, tag = "5")]
    pub incoming_amount_msat: u64,
    /// The incoming htlc expiry.
    #[prost(uint32, tag = "6")]
    pub incoming_expiry: u32,
    ///
    /// The htlc payment hash. This value is not guaranteed to be unique per
    /// request.
    #[prost(bytes = "vec", tag = "2")]
    pub payment_hash: ::prost::alloc::vec::Vec<u8>,
    /// The requested outgoing channel id for this forwarded htlc. Because of
    /// non-strict forwarding, this isn't necessarily the channel over which the
    /// packet will be forwarded eventually. A different channel to the same peer
    /// may be selected as well.
    #[prost(uint64, tag = "7")]
    pub outgoing_requested_chan_id: u64,
    /// The outgoing htlc amount.
    #[prost(uint64, tag = "3")]
    pub outgoing_amount_msat: u64,
    /// The outgoing htlc expiry.
    #[prost(uint32, tag = "4")]
    pub outgoing_expiry: u32,
    /// Any custom records that were present in the payload.
    #[prost(map = "uint64, bytes", tag = "8")]
    pub custom_records: ::std::collections::HashMap<u64, ::prost::alloc::vec::Vec<u8>>,
    /// The onion blob for the next hop
    #[prost(bytes = "vec", tag = "9")]
    pub onion_blob: ::prost::alloc::vec::Vec<u8>,
    /// The block height at which this htlc will be auto-failed to prevent the
    /// channel from force-closing.
    #[prost(int32, tag = "10")]
    pub auto_fail_height: i32,
}
/// *
/// ForwardHtlcInterceptResponse enables the caller to resolve a previously hold
/// forward. The caller can choose either to:
/// - `Resume`: Execute the default behavior (usually forward).
/// - `Reject`: Fail the htlc backwards.
/// - `Settle`: Settle this htlc with a given preimage.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ForwardHtlcInterceptResponse {
    /// *
    /// The key of this forwarded htlc. It defines the incoming channel id and
    /// the index in this channel.
    #[prost(message, optional, tag = "1")]
    pub incoming_circuit_key: ::core::option::Option<CircuitKey>,
    /// The resolve action for this intercepted htlc.
    #[prost(enumeration = "ResolveHoldForwardAction", tag = "2")]
    pub action: i32,
    /// The preimage in case the resolve action is Settle.
    #[prost(bytes = "vec", tag = "3")]
    pub preimage: ::prost::alloc::vec::Vec<u8>,
    /// Encrypted failure message in case the resolve action is Fail.
    ///
    /// If failure_message is specified, the failure_code field must be set
    /// to zero.
    #[prost(bytes = "vec", tag = "4")]
    pub failure_message: ::prost::alloc::vec::Vec<u8>,
    /// Return the specified failure code in case the resolve action is Fail. The
    /// message data fields are populated automatically.
    ///
    /// If a non-zero failure_code is specified, failure_message must not be set.
    ///
    /// For backwards-compatibility reasons, TEMPORARY_CHANNEL_FAILURE is the
    /// default value for this field.
    #[prost(enumeration = "super::lnrpc::failure::FailureCode", tag = "5")]
    pub failure_code: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateChanStatusRequest {
    #[prost(message, optional, tag = "1")]
    pub chan_point: ::core::option::Option<super::lnrpc::ChannelPoint>,
    #[prost(enumeration = "ChanStatusAction", tag = "2")]
    pub action: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateChanStatusResponse {}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FailureDetail {
    Unknown = 0,
    NoDetail = 1,
    OnionDecode = 2,
    LinkNotEligible = 3,
    OnChainTimeout = 4,
    HtlcExceedsMax = 5,
    InsufficientBalance = 6,
    IncompleteForward = 7,
    HtlcAddFailed = 8,
    ForwardsDisabled = 9,
    InvoiceCanceled = 10,
    InvoiceUnderpaid = 11,
    InvoiceExpiryTooSoon = 12,
    InvoiceNotOpen = 13,
    MppInvoiceTimeout = 14,
    AddressMismatch = 15,
    SetTotalMismatch = 16,
    SetTotalTooLow = 17,
    SetOverpaid = 18,
    UnknownInvoice = 19,
    InvalidKeysend = 20,
    MppInProgress = 21,
    CircularRoute = 22,
}
impl FailureDetail {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FailureDetail::Unknown => "UNKNOWN",
            FailureDetail::NoDetail => "NO_DETAIL",
            FailureDetail::OnionDecode => "ONION_DECODE",
            FailureDetail::LinkNotEligible => "LINK_NOT_ELIGIBLE",
            FailureDetail::OnChainTimeout => "ON_CHAIN_TIMEOUT",
            FailureDetail::HtlcExceedsMax => "HTLC_EXCEEDS_MAX",
            FailureDetail::InsufficientBalance => "INSUFFICIENT_BALANCE",
            FailureDetail::IncompleteForward => "INCOMPLETE_FORWARD",
            FailureDetail::HtlcAddFailed => "HTLC_ADD_FAILED",
            FailureDetail::ForwardsDisabled => "FORWARDS_DISABLED",
            FailureDetail::InvoiceCanceled => "INVOICE_CANCELED",
            FailureDetail::InvoiceUnderpaid => "INVOICE_UNDERPAID",
            FailureDetail::InvoiceExpiryTooSoon => "INVOICE_EXPIRY_TOO_SOON",
            FailureDetail::InvoiceNotOpen => "INVOICE_NOT_OPEN",
            FailureDetail::MppInvoiceTimeout => "MPP_INVOICE_TIMEOUT",
            FailureDetail::AddressMismatch => "ADDRESS_MISMATCH",
            FailureDetail::SetTotalMismatch => "SET_TOTAL_MISMATCH",
            FailureDetail::SetTotalTooLow => "SET_TOTAL_TOO_LOW",
            FailureDetail::SetOverpaid => "SET_OVERPAID",
            FailureDetail::UnknownInvoice => "UNKNOWN_INVOICE",
            FailureDetail::InvalidKeysend => "INVALID_KEYSEND",
            FailureDetail::MppInProgress => "MPP_IN_PROGRESS",
            FailureDetail::CircularRoute => "CIRCULAR_ROUTE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "NO_DETAIL" => Some(Self::NoDetail),
            "ONION_DECODE" => Some(Self::OnionDecode),
            "LINK_NOT_ELIGIBLE" => Some(Self::LinkNotEligible),
            "ON_CHAIN_TIMEOUT" => Some(Self::OnChainTimeout),
            "HTLC_EXCEEDS_MAX" => Some(Self::HtlcExceedsMax),
            "INSUFFICIENT_BALANCE" => Some(Self::InsufficientBalance),
            "INCOMPLETE_FORWARD" => Some(Self::IncompleteForward),
            "HTLC_ADD_FAILED" => Some(Self::HtlcAddFailed),
            "FORWARDS_DISABLED" => Some(Self::ForwardsDisabled),
            "INVOICE_CANCELED" => Some(Self::InvoiceCanceled),
            "INVOICE_UNDERPAID" => Some(Self::InvoiceUnderpaid),
            "INVOICE_EXPIRY_TOO_SOON" => Some(Self::InvoiceExpiryTooSoon),
            "INVOICE_NOT_OPEN" => Some(Self::InvoiceNotOpen),
            "MPP_INVOICE_TIMEOUT" => Some(Self::MppInvoiceTimeout),
            "ADDRESS_MISMATCH" => Some(Self::AddressMismatch),
            "SET_TOTAL_MISMATCH" => Some(Self::SetTotalMismatch),
            "SET_TOTAL_TOO_LOW" => Some(Self::SetTotalTooLow),
            "SET_OVERPAID" => Some(Self::SetOverpaid),
            "UNKNOWN_INVOICE" => Some(Self::UnknownInvoice),
            "INVALID_KEYSEND" => Some(Self::InvalidKeysend),
            "MPP_IN_PROGRESS" => Some(Self::MppInProgress),
            "CIRCULAR_ROUTE" => Some(Self::CircularRoute),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PaymentState {
    ///
    /// Payment is still in flight.
    InFlight = 0,
    ///
    /// Payment completed successfully.
    Succeeded = 1,
    ///
    /// There are more routes to try, but the payment timeout was exceeded.
    FailedTimeout = 2,
    ///
    /// All possible routes were tried and failed permanently. Or were no
    /// routes to the destination at all.
    FailedNoRoute = 3,
    ///
    /// A non-recoverable error has occurred.
    FailedError = 4,
    ///
    /// Payment details incorrect (unknown hash, invalid amt or
    /// invalid final cltv delta)
    FailedIncorrectPaymentDetails = 5,
    ///
    /// Insufficient local balance.
    FailedInsufficientBalance = 6,
}
impl PaymentState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PaymentState::InFlight => "IN_FLIGHT",
            PaymentState::Succeeded => "SUCCEEDED",
            PaymentState::FailedTimeout => "FAILED_TIMEOUT",
            PaymentState::FailedNoRoute => "FAILED_NO_ROUTE",
            PaymentState::FailedError => "FAILED_ERROR",
            PaymentState::FailedIncorrectPaymentDetails => "FAILED_INCORRECT_PAYMENT_DETAILS",
            PaymentState::FailedInsufficientBalance => "FAILED_INSUFFICIENT_BALANCE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "IN_FLIGHT" => Some(Self::InFlight),
            "SUCCEEDED" => Some(Self::Succeeded),
            "FAILED_TIMEOUT" => Some(Self::FailedTimeout),
            "FAILED_NO_ROUTE" => Some(Self::FailedNoRoute),
            "FAILED_ERROR" => Some(Self::FailedError),
            "FAILED_INCORRECT_PAYMENT_DETAILS" => Some(Self::FailedIncorrectPaymentDetails),
            "FAILED_INSUFFICIENT_BALANCE" => Some(Self::FailedInsufficientBalance),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ResolveHoldForwardAction {
    Settle = 0,
    Fail = 1,
    Resume = 2,
}
impl ResolveHoldForwardAction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ResolveHoldForwardAction::Settle => "SETTLE",
            ResolveHoldForwardAction::Fail => "FAIL",
            ResolveHoldForwardAction::Resume => "RESUME",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SETTLE" => Some(Self::Settle),
            "FAIL" => Some(Self::Fail),
            "RESUME" => Some(Self::Resume),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChanStatusAction {
    Enable = 0,
    Disable = 1,
    Auto = 2,
}
impl ChanStatusAction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ChanStatusAction::Enable => "ENABLE",
            ChanStatusAction::Disable => "DISABLE",
            ChanStatusAction::Auto => "AUTO",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ENABLE" => Some(Self::Enable),
            "DISABLE" => Some(Self::Disable),
            "AUTO" => Some(Self::Auto),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod router_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Router is a service that offers advanced interaction with the router
    /// subsystem of the daemon.
    #[derive(Debug, Clone)]
    pub struct RouterClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RouterClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> RouterClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> RouterClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            RouterClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        ///
        /// SendPaymentV2 attempts to route a payment described by the passed
        /// PaymentRequest to the final destination. The call returns a stream of
        /// payment updates. When using this RPC, make sure to set a fee limit, as the
        /// default routing fee limit is 0 sats. Without a non-zero fee limit only
        /// routes without fees will be attempted which often fails with
        /// FAILURE_REASON_NO_ROUTE.
        pub async fn send_payment_v2(
            &mut self,
            request: impl tonic::IntoRequest<super::SendPaymentRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::super::lnrpc::Payment>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/routerrpc.Router/SendPaymentV2");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("routerrpc.Router", "SendPaymentV2"));
            self.inner.server_streaming(req, path, codec).await
        }
        /// lncli: `trackpayment`
        /// TrackPaymentV2 returns an update stream for the payment identified by the
        /// payment hash.
        pub async fn track_payment_v2(
            &mut self,
            request: impl tonic::IntoRequest<super::TrackPaymentRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::super::lnrpc::Payment>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/routerrpc.Router/TrackPaymentV2");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("routerrpc.Router", "TrackPaymentV2"));
            self.inner.server_streaming(req, path, codec).await
        }
        ///
        /// TrackPayments returns an update stream for every payment that is not in a
        /// terminal state. Note that if payments are in-flight while starting a new
        /// subscription, the start of the payment stream could produce out-of-order
        /// and/or duplicate events. In order to get updates for every in-flight
        /// payment attempt make sure to subscribe to this method before initiating any
        /// payments.
        pub async fn track_payments(
            &mut self,
            request: impl tonic::IntoRequest<super::TrackPaymentsRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::super::lnrpc::Payment>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/routerrpc.Router/TrackPayments");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("routerrpc.Router", "TrackPayments"));
            self.inner.server_streaming(req, path, codec).await
        }
        ///
        /// EstimateRouteFee allows callers to obtain a lower bound w.r.t how much it
        /// may cost to send an HTLC to the target end destination.
        pub async fn estimate_route_fee(
            &mut self,
            request: impl tonic::IntoRequest<super::RouteFeeRequest>,
        ) -> std::result::Result<tonic::Response<super::RouteFeeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/routerrpc.Router/EstimateRouteFee");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("routerrpc.Router", "EstimateRouteFee"));
            self.inner.unary(req, path, codec).await
        }
        ///
        /// Deprecated, use SendToRouteV2. SendToRoute attempts to make a payment via
        /// the specified route. This method differs from SendPayment in that it
        /// allows users to specify a full route manually. This can be used for
        /// things like rebalancing, and atomic swaps. It differs from the newer
        /// SendToRouteV2 in that it doesn't return the full HTLC information.
        pub async fn send_to_route(
            &mut self,
            request: impl tonic::IntoRequest<super::SendToRouteRequest>,
        ) -> std::result::Result<tonic::Response<super::SendToRouteResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/routerrpc.Router/SendToRoute");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("routerrpc.Router", "SendToRoute"));
            self.inner.unary(req, path, codec).await
        }
        ///
        /// SendToRouteV2 attempts to make a payment via the specified route. This
        /// method differs from SendPayment in that it allows users to specify a full
        /// route manually. This can be used for things like rebalancing, and atomic
        /// swaps.
        pub async fn send_to_route_v2(
            &mut self,
            request: impl tonic::IntoRequest<super::SendToRouteRequest>,
        ) -> std::result::Result<tonic::Response<super::super::lnrpc::HtlcAttempt>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/routerrpc.Router/SendToRouteV2");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("routerrpc.Router", "SendToRouteV2"));
            self.inner.unary(req, path, codec).await
        }
        /// lncli: `resetmc`
        /// ResetMissionControl clears all mission control state and starts with a clean
        /// slate.
        pub async fn reset_mission_control(
            &mut self,
            request: impl tonic::IntoRequest<super::ResetMissionControlRequest>,
        ) -> std::result::Result<tonic::Response<super::ResetMissionControlResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/routerrpc.Router/ResetMissionControl");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("routerrpc.Router", "ResetMissionControl"));
            self.inner.unary(req, path, codec).await
        }
        /// lncli: `querymc`
        /// QueryMissionControl exposes the internal mission control state to callers.
        /// It is a development feature.
        pub async fn query_mission_control(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryMissionControlRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryMissionControlResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/routerrpc.Router/QueryMissionControl");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("routerrpc.Router", "QueryMissionControl"));
            self.inner.unary(req, path, codec).await
        }
        /// lncli: `importmc`
        /// XImportMissionControl is an experimental API that imports the state provided
        /// to the internal mission control's state, using all results which are more
        /// recent than our existing values. These values will only be imported
        /// in-memory, and will not be persisted across restarts.
        pub async fn x_import_mission_control(
            &mut self,
            request: impl tonic::IntoRequest<super::XImportMissionControlRequest>,
        ) -> std::result::Result<tonic::Response<super::XImportMissionControlResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/routerrpc.Router/XImportMissionControl");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("routerrpc.Router", "XImportMissionControl"));
            self.inner.unary(req, path, codec).await
        }
        /// lncli: `getmccfg`
        /// GetMissionControlConfig returns mission control's current config.
        pub async fn get_mission_control_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMissionControlConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetMissionControlConfigResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/routerrpc.Router/GetMissionControlConfig");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "routerrpc.Router",
                "GetMissionControlConfig",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// lncli: `setmccfg`
        /// SetMissionControlConfig will set mission control's config, if the config
        /// provided is valid.
        pub async fn set_mission_control_config(
            &mut self,
            request: impl tonic::IntoRequest<super::SetMissionControlConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetMissionControlConfigResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/routerrpc.Router/SetMissionControlConfig");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "routerrpc.Router",
                "SetMissionControlConfig",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// lncli: `queryprob`
        /// Deprecated. QueryProbability returns the current success probability
        /// estimate for a given node pair and amount. The call returns a zero success
        /// probability if no channel is available or if the amount violates min/max
        /// HTLC constraints.
        pub async fn query_probability(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryProbabilityRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryProbabilityResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/routerrpc.Router/QueryProbability");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("routerrpc.Router", "QueryProbability"));
            self.inner.unary(req, path, codec).await
        }
        /// lncli: `buildroute`
        /// BuildRoute builds a fully specified route based on a list of hop public
        /// keys. It retrieves the relevant channel policies from the graph in order to
        /// calculate the correct fees and time locks.
        /// Note that LND will use its default final_cltv_delta if no value is supplied.
        /// Make sure to add the correct final_cltv_delta depending on the invoice
        /// restriction. Moreover the caller has to make sure to provide the
        /// payment_addr if the route is paying an invoice which signaled it.
        pub async fn build_route(
            &mut self,
            request: impl tonic::IntoRequest<super::BuildRouteRequest>,
        ) -> std::result::Result<tonic::Response<super::BuildRouteResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/routerrpc.Router/BuildRoute");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("routerrpc.Router", "BuildRoute"));
            self.inner.unary(req, path, codec).await
        }
        ///
        /// SubscribeHtlcEvents creates a uni-directional stream from the server to
        /// the client which delivers a stream of htlc events.
        pub async fn subscribe_htlc_events(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeHtlcEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::HtlcEvent>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/routerrpc.Router/SubscribeHtlcEvents");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("routerrpc.Router", "SubscribeHtlcEvents"));
            self.inner.server_streaming(req, path, codec).await
        }
        ///
        /// Deprecated, use SendPaymentV2. SendPayment attempts to route a payment
        /// described by the passed PaymentRequest to the final destination. The call
        /// returns a stream of payment status updates.
        pub async fn send_payment(
            &mut self,
            request: impl tonic::IntoRequest<super::SendPaymentRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::PaymentStatus>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/routerrpc.Router/SendPayment");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("routerrpc.Router", "SendPayment"));
            self.inner.server_streaming(req, path, codec).await
        }
        ///
        /// Deprecated, use TrackPaymentV2. TrackPayment returns an update stream for
        /// the payment identified by the payment hash.
        pub async fn track_payment(
            &mut self,
            request: impl tonic::IntoRequest<super::TrackPaymentRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::PaymentStatus>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/routerrpc.Router/TrackPayment");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("routerrpc.Router", "TrackPayment"));
            self.inner.server_streaming(req, path, codec).await
        }
        /// *
        /// HtlcInterceptor dispatches a bi-directional streaming RPC in which
        /// Forwarded HTLC requests are sent to the client and the client responds with
        /// a boolean that tells LND if this htlc should be intercepted.
        /// In case of interception, the htlc can be either settled, cancelled or
        /// resumed later by using the ResolveHoldForward endpoint.
        pub async fn htlc_interceptor(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::ForwardHtlcInterceptResponse>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::ForwardHtlcInterceptRequest>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/routerrpc.Router/HtlcInterceptor");
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("routerrpc.Router", "HtlcInterceptor"));
            self.inner.streaming(req, path, codec).await
        }
        /// lncli: `updatechanstatus`
        /// UpdateChanStatus attempts to manually set the state of a channel
        /// (enabled, disabled, or auto). A manual "disable" request will cause the
        /// channel to stay disabled until a subsequent manual request of either
        /// "enable" or "auto".
        pub async fn update_chan_status(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateChanStatusRequest>,
        ) -> std::result::Result<tonic::Response<super::UpdateChanStatusResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/routerrpc.Router/UpdateChanStatus");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("routerrpc.Router", "UpdateChanStatus"));
            self.inner.unary(req, path, codec).await
        }
    }
}
