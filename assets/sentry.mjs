import * as Sentry from "@sentry/browser";
import { wasmIntegration } from "@sentry/wasm";
Sentry.init({
  dsn: "https://76dc2cedba0032f2adeda47f0f6972cb@o4509961437970432.ingest.us.sentry.io/4509961442557952",
  // Adds request headers and IP for users, for more info visit:
  // https://docs.sentry.io/platforms/javascript/guides/wasm/configuration/options/#sendDefaultPii
  sendDefaultPii: true,
  integrations: [
    //  performance
    Sentry.browserTracingIntegration(),
    //  performance
    wasmIntegration(),
  ],
  //  performance
  // Set tracesSampleRate to 1.0 to capture 100%
  // of transactions for tracing.
  // We recommend adjusting this value in production
  // Learn more at
  // https://docs.sentry.io/platforms/javascript/guides/wasm/configuration/options/#tracesSampleRate
  tracesSampleRate: 1.0,
  // Set `tracePropagationTargets` to control for which URLs trace propagation should be enabled
  tracePropagationTargets: ["localhost", /^https:\/\/yourserver\.io\/api/],
  //  performance
  //  logs
  // Enable logs to be sent to Sentry
  enableLogs: true,
  //  logs
});
