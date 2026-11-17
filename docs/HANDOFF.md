# 📜 Arctypon OS 协作合約 v2（Handoff & Constitution）
> 取代 v1。本檔隨 repo 版本化；每個 Phase 完成、架構變更、停歇 >24h 前必須更新並 commit。
> v1→v2 變更清單見附錄 A。

## 0. 核心憲章與雙軌哲學
- **雙軌邊界**：`host/`（std）與 `crates/`+`kernel/`（no_std）嚴格分離。Iggy、Rivet、GPUI、wasmtime 等重型生態僅宿主層。
- **alloc 政策**（取代含糊的「預設 alloc-free」）：no_std 強制；alloc 與否**逐 crate 顯式聲明**於 crate 頂層 doc：
  `//! alloc: yes|no — <理由>`。現狀：arctypon-core = yes（Mailbox）；arctypon(kernel) = yes（OSTD global allocator）；types/sim/abi = no。
- **AI 首席架構師**：用戶做宏觀決策；AI 負責實作、測試推演、依賴搜尋。
- **極度模組化**：一功能一 crate、一子功能一檔；每次交付 ≤2 檔、單檔 ≤200 行。
  **升格規則**（防微 crate 爆炸）：先在既有 crate 內以 module 成長，超過 ~3 檔或需獨立 MSRV/授權時才升格為 crate。
- **零複製貼上**：交付一律 `cat << 'EOF' > path` heredoc（先 `mkdir -p`）＋一鍵命令＋結尾驗證命令。
- **執行前查庫**（具體命令，違者視同重複定義紅線）：
  cargo metadata --no-deps --format-version 1 | jq -r '.packages[].name'
  rg -n "pub (struct|enum|trait|type) " crates/ kernel/ | rg -i "<關鍵詞>"

## 1. 環境與工具鏈
- Host：stable 1.100.0 / x86_64-unknown-linux-gnu（ASUS TUF A15, Ubuntu）。
- Kernel：nightly-2026-07-21（OSDK 0.19.0 釘版於 kernel/rust-toolchain.toml）；主目標 x86_64-unknown-none。
  riscv64imac-unknown-none-elf 為 Phase-gated：入場條件＝x86_64 `cargo osdk test` 連續 3 個 commit 綠。
- **依賴鎖定**：crates.io 優先；git 依賴必 `rev = "<hash>"`，禁 `branch`。
- **授權白名單**（新增，呼應 IP 策略）：MIT / Apache-2.0 / BSD-2/3 / ISC / Zlib 可用；MPL-2.0 僅限未修改依賴（ostd）；GPL/AGPL/SSPL 禁用；其他＝停下問用戶。
- **供應鏈**：`.cargo/config.toml` 之 `global-min-publish-age = "7 days"` 常駐。
  例外 SOP：`CARGO_RESOLVER_INCOMPATIBLE_PUBLISH_AGE=allow cargo update --precise <crate>@<ver>`，理由寫進 commit message。
- 動 kernel 前的交付必先跑附錄 B `check_env.sh`，依回顯決定下一步。

## 2. 架構約束與紅線
- **目錄樹（與現實對齊版）**：
  arctypon/                          # workspace；已發布 arctypon、arctypon-core 0.0.0
  ├── kernel/                        # OSDK kernel；package 名 = "arctypon"（名稱保留，禁改名）
  │   ├── Cargo.toml / OSDK.toml / rust-toolchain.toml
  │   └── src/lib.rs                 # #[ostd::main] 入口；split 優先用 src/{boot,sched,hal}/ module
  ├── crates/
  │   ├── arctypon-core/             # ✅ 0.0.0 — Actor trait + allocator-generic Mailbox
  │   ├── arctypon-types/            # ⏳ P1 — ID、Fixed 包裝、Error、seeded PRNG（alloc: no）
  │   ├── arctypon-sim/              # ⏳ P1 — 確定性網格狀態機
  │   ├── arctypon-abi/              # ⏳ P2 — host↔kernel/WASM 共享布局：repr(C) + 靜態 assert
  │   └── arctypon-runtime/          # ⏳ P2 — 排程器、plugin 沙箱（wasmi 為 no_std 候選）
  ├── host/                          # ⏳ P3 — std 層
  └── docs/HANDOFF.md                # 本檔
- **新 crate 建立檢查單**：加入 workspace members；當場決定 publish 或保留；首發即填 keywords+categories；複製 LICENSE-MIT/LICENSE-APACHE 進 crate 目錄。
- **安全鐵律（矛盾已解）**：arctypon 自有程式碼**全部** `#![forbid(unsafe_code)]`——含 kernel crates（OSTD 提供 safe API，此即 framekernel 承諾）。
  例外允許清單：**目前為空**。若 OSTD safe API 不足＝開 issue → 用戶批准 → 單一 module `#![allow(unsafe_code)]`＋逐塊 `// SAFETY:`；workspace lint `unsafe_code = "deny"` 為後備閘。
- **確定性約束**（arctypon-sim 及任何狀態機）：
  禁 f32/f64 → `fixed::FixedI32::<U16>`（Fixed<16,16>，採用時鎖精確版本）；
  禁 std::time → 顯式 tick: u64 傳參；
  禁 HashMap → 預設 BTreeMap；確需 hash 用 hashbrown + 固定 Seed 的 BuildHasher（Seed 定義於 arctypon-types）；
  禁 rand crate → 自製 xorshift64*/PCG（seed 顯式）；
  **重放測試強制**：(seed, inputs) → state hash 在 host 與 kernel 建置間逐位元相同。

## 3. 測試、驗收與修復閉環
- 硬性門禁（宣稱完成前全綠）：
  cargo fmt --check
  cargo clippy --workspace --all-targets -- -D warnings
  cargo test --workspace
  cargo osdk test            # 僅當觸及 kernel/
- clippy 增補（workspace lints）：unwrap_used / expect_used / panicking_macros = "deny"；
  測試模組頂部 `#![allow(clippy::unwrap_used, clippy::expect_used)]` 豁免（保留測試慣例）。
- **【AI 驗收報告】**格式：🟢/🔴 狀態｜🎯 核心功能｜🧠 推演｜🛠 驗證命令（附真實輸出節錄）｜📦 交付檔案｜🔗 commit hash｜🚧 門禁結果表。
- **AI 自主修復上限 3 次**／同一門禁；超限＝停機輸出 🔴 報告（完整錯誤＋假設清單）。
  禁 silently 跳過門禁、禁改 lint 等級或工具鏈釘版來變綠——後兩者屬架構變更，需用戶批准＋更新本檔。

## 4. 狀態快照（Handoff Protocol）
- 位置：本檔（repo 內）。~/Arctypon_doc/ 廢除。
- 必備區段：① 進度條表（crate｜狀態｜版本｜commit）② 工具鏈釘版 ③ 未決問題 Q#（open/closed、owner、決策紀錄）④ 下一步行動（精確到 crate＋檔路徑＋驗收標準）⑤ 證據鏈附錄。

## 附錄 A：v1→v2 變更摘要
1. 目錄樹與已發布 crate 對齊；kernel 禁改名（名稱保留）。
2. 解 unsafe 矛盾：全自有碼 forbid；例外走批准制允許清單。
3. 解 alloc 矛盾：逐 crate 顯式聲明制。
4. 解 clippy/unwrap 矛盾：deny + 測試模組定向豁免。
5. 交接檔移入 repo（證據鏈）。
6. 「查庫」「確定性」給出具體命令與 crate 選型（fixed/BTreeMap/自製 PRNG）。
7. 新增授權白名單、新 crate 檢查單、微 crate 升格規則。
8. 新增 AI 修復上限與停機條件（防無限迴圈）。
9. riscv64 改為 Phase-gated 並給入場條件。
10. 驗收報告增 📦/🔗/🚧 三欄。

## 附錄 B：check_env.sh
#!/usr/bin/env bash
set -uo pipefail
say(){ printf '%-24s %s\n' "$1" "$2"; }
say "rustc(host)"   "$(rustc +stable --version 2>&1 || echo MISSING)"
say "rustc(kernel)" "$(rustc +nightly-2026-07-21 --version 2>&1 || echo MISSING)"
say "cargo-osdk"    "$(cargo osdk --version 2>&1 | head -n1 || echo MISSING)"
say "qemu"          "$(qemu-system-x86_64 --version 2>/dev/null | head -n1 || echo MISSING)"
say "ovmf_code"     "$(ls /usr/share/OVMF/OVMF_CODE_4M.fd 2>/dev/null || echo MISSING)"
say "ovmf_vars"     "$(ls /usr/share/OVMF/OVMF_VARS_4M.fd 2>/dev/null || echo MISSING)"
say "grub-mkrescue" "$(command -v grub-mkrescue || echo MISSING)"
say "xorriso"       "$(command -v xorriso || echo MISSING)"
say "workspace"     "$(cargo metadata --no-deps --format-version 1 2>/dev/null | jq -r '.packages|length' || echo BROKEN) packages"

## 附錄 C：當前快照（v2 初始）
進度條：
| 項目 | 狀態 | 版本/commit |
|---|---|---|
| arctypon-core（Mailbox sketch＋測試） | ✅ published | 0.0.0 |
| arctypon（kernel placeholder） | ✅ published | 0.0.0 |
| workspace 骨架＋雙授權＋cooldown | ✅ pushed | 4f48729 |
| cargo osdk 首次開機 | ⏳ 阻於 Q3 | — |
| types / sim / abi / runtime / host | ⏳ 未開始 | — |
未決問題：
Q1 host UI 路線（GPUI vs Web）— open（owner: 用戶；P3 入場前決）
Q2 allocator_api 在釘版 nightly 的 gate — closed（arctypon-core `nightly` feature 解）
Q3 OSDK.toml 之 OVMF 路徑（裸機 Ubuntu）— open（阻首次開機）
Q4 rust-analyzer 對 core/lib.rs 報 2 錯 — open（cosmetic；cargo 全綠）
Q5 kernel crate 的 docs.rs 建置（ostd 目標）— open（cosmetic）
Q6 crates.io 保留 arctypon-runtime / arctypon-macros — open
Q7 polish：topics、org website、per-crate LICENSE、badge、tag v0.0.0 — open
下一步行動：
1. 修 kernel/OSDK.toml OVMF 路徑 → `cargo osdk run`；驗收＝QEMU 印出 hello 行。（解 Q3）
2. 建 crates/arctypon-types（alloc: no）：ActorId/CellId newtype、Error enum、Fixed 包裝、seeded PRNG＋重放測試。
3. commit 本檔＋.vscode/settings.json；執行 Q7 polish 清單。
證據鏈：org 2026-11-16T12:53Z｜commit 4f48729｜crates.io core 04:03:22Z（3552f4ce…）｜arctypon 04:04:14Z（08da4ae4…）

## 附錄 D：Q3 決策紀錄（2026-11-17）
- 根因 1：keywords 誤置 [lints] → cargo metadata 失敗 → OSDK manifest panic（已修）。
- 根因 2：OSDK 子進程 shim 在 target/ 下找不到 rust-toolchain.toml → 回落 stable → `-Z` 被拒；`+nightly` 不外傳，僅 RUSTUP_TOOLCHAIN 外傳。
- SOP：osdk 一律 `./run.sh <args>`；Docker 備援 tag 規則 `<ver>-<yyyymmdd>`（0.19.0 → 0.19.0-20261012）＋ `--scheme docker`。
- 裸機 OVMF：CODE 唯讀 /usr/share/OVMF/OVMF_CODE_4M.fd；VARS 可寫副本 ~/.cache/arctypon/OVMF_VARS.fd。
- Nix shell（PR asterinas#3548）暫不採用：osdk-new 專案硬編碼 Docker firmware 路徑＋首次 ~10GB。
- Q3：open，待 ./run.sh run 實測後關閉。
- Q3 追加（19:37）：run.sh 之 RUSTUP_TOOLCHAIN 實測仍落 stable（H1/H2/H3 待診）；改採 directory override：根=nightly-2026-07-21、crates/arctypon-core=stable。override 依 cwd 查表，對 env 清理免疫。

## 附錄 D：Q3 決策紀錄（完整鏈，2026-11-17）
狀態：**關閉**（首次開機成功，recv = Some(42) 驗證）。

修復鏈（按時間序）：
1. kernel/Cargo.toml：keywords 誤置 [lints] → cargo metadata 失敗 → OSDK manifest panic。修正：移回 [package]。
2. 路徑中文字元：OSDK 0.19.0 在含非 ASCII 路徑下 fs::metadata 失敗。使用者 mv 文件 → Documents + symlink 解決。
3. cargo 子進程工具鏈解析：
   - `cargo +nightly` 不外傳子進程（shim 不傳承 `+toolchain` flag）
   - `RUSTUP_TOOLCHAIN` env 實測亦失效（H1：非 shim 的真實 cargo 在 PATH 優先；H2/H3：env sanitize）
   - **結構性解**：`rustup override set` 依 cwd 生效，對 base crate（target/ 下）亦覆蓋
4. 雙軌 override 策略：
   - 根 /home/luna/Documents/arctypon → nightly-2026-07-21（kernel 路徑）
   - crates/arctypon-core → stable-x86_64-unknown-linux-gnu（宿主測試路徑）
   - workspace 測試靠 feature 統一在 nightly 下編譯 core
5. feature gate 缺漏：kernel/src/lib.rs 缺 `#![feature(allocator_api)]`（stable 1.100 已穩，但釘版 nightly-2026-07-21 ≈ Rust 1.99 仍 unstable）。加一行修復。
6. workspace lints 錯置：`unwrap_used`/`panic` 是 clippy lint，誤置 `[lints.rust]` 觸發 rustc E0602。移至 `[workspace.lints.clippy]`。
7. OVMF VARS 可寫性：QEMU pflash unit=1 需可寫檔案；建 ~/.cache/arctypon/OVMF_VARS.fd 副本。
8. OSDK.toml 雙 scheme：預設（裸機路徑）+ `[scheme.docker.qemu]`（容器路徑）。
9. run.sh 統一入口：`export RUSTUP_TOOLCHAIN=nightly-2026-07-21` + `cd kernel && cargo osdk "$@"`。

環境事實：
- /home/luna/Documents/arctypon 實際路徑（`~/文件` 是 symlink）
- 使用者已有 /home/luna/warehouse/arc-osdk 的獨立 override（同 nightly）

未納入：PR asterinas#3548 的 Nix shell（osdk-new 專案硬編碼 Docker firmware 路徑＋首次 ~10GB 下載）。

日誌噪音清單（已知無害，不必修）：
- TCG CPUID feature warnings × 17
- GRUB "no suitable video mode"（走 serial 不影響）
- IOMMU NoIommu（未啟用 DMA）
- `ignoring registry.global-min-publish-age without -Zmin-publish-age`（nightly 需 -Z 才生效）

## 附錄 E：atomr 模式提取（2026-11-17，僅模式、零依賴）
- tell() 傳輸無關 → Dispatcher trait（enqueue(&[u8]) + placement()）；實作序：Mailbox → HostShmem(ABI) → Accel(OSTD DMA)。
- tell() 零分配：postcard 序列化至棧上 [u8; 256]；超限 = InvalidPayload。
- 監督/反壓/可觀測三一致：Supervisor trait、BackpressurePolicy enum、ActorStats 原子計數，皆與 dispatcher 解耦。
- Placement enum 由 runtime 於 spawn 顯式指定；單 binary 多 placement（雙軌制詮釋 atomr「不寫兩份程式」）。
- 交付序：dispatch.rs（core）→ supervisor.rs（runtime）→ stats（runtime）→ HostShmem（abi, Phase 3）。
