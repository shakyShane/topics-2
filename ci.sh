set -euo pipefail

cargo run --bin ir_ts_types > ir/typescript/ir_ts_types.ts
cargo run --bin ir_ts_types > frontend-svelte/__generated__/ir_ts_types.ts
cargo run --bin ir_json_stubs > frontend-svelte/__generated__/output.json