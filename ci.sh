set -euo pipefail

cargo run --bin ir_ts_types > ir/typescript/ir_ts_types.ts
#cargo run --bin ir_ts_types > frontend-svelte/__generated__/ir_ts_types.ts
#cargo run --bin ir_json_stubs > frontend-svelte/static/output.json
cargo run --bin ir_ts_types > frontend-next/__generated__/ir_ts_types.ts
cargo run --bin ir_json_stubs > frontend-next/public/output.json