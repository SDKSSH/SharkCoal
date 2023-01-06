struct Config {
    sentry : SentryConfig,
    name : String,
    port : u16
}

struct SentryConfig {
    useSentry : bool,
    sentryUrl : String
}