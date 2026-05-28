# Xin Language - Standar Pengembangan Kode

Dokumen ini menetapkan panduan standar ketat untuk kode, struktur, dan praktik pengembangan project Xin Language.

---

## 1. Prinsip Dasar (The "Why")

- **Eksplisit di atas implisit** – semua kontrak, tipe, error, dan alur data harus terlihat jelas tanpa sihir makro.
- **Zero-panic production** – perpustakaan inti tidak boleh menggunakan unwrap, expect, panic!; semua kegagalan diketik kuat.
- **Modularitas ekstrem** – setiap unit bisnis, domain, dan infrastruktur dipisah ke crate sendiri, dengan API publik yang dijaga ketat.
- **Lapisan Clean Architecture** – Domain (entities, value objects) tidak bergantung pada apa pun; Application (use case) hanya bergantung pada Domain; Infrastructure mengimplementasi port; Interface menangani I/O.
- **Biaya awal tinggi bisa diterima** – review ketat, test coverage 90%+, dokumentasi wajib, ADR (Architecture Decision Records) untuk setiap keputusan penting.

---

## 2. Struktur Folder (Monorepo Workspace)

Gunakan Cargo workspace dengan kedalaman bersarang hingga 10+ level untuk memisahkan bounded context, fitur, dan lapisan.

```
project-root/
├── Cargo.toml                         # workspace utama
├── rust-toolchain.toml                # versi toolchain tetap, misal stable-2026
├── deny.toml                          # cargo-deny (lisensi, advisories)
├── .rustfmt.toml                      # konfigurasi format ketat
├── clippy.toml                        # lints ekstra
├── docs/
│   ├── adr/                           # Architecture Decision Records
│   │   └── ...
│   └── diagrams/                      # C4, sequence diagram (opsional)
├── crates/
│   ├── core/                          # kernel: tipe primitif, trait dasar
│   │   └── ...
│   ├── domain/                        # tiap bounded context dipisah crate
│   │   ├── user/
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       ├── model/             # domain model
│   │   │       │   ├── entity/
│   │   │       │   │   ├── mod.rs
│   │   │       │   │   ├── user.rs
│   │   │       │   │   └── profile.rs
│   │   │       │   ├── value_object/
│   │   │       │   │   ├── email.rs
│   │   │       │   │   ├── password/
│   │   │       │   │   │   ├── mod.rs
│   │   │       │   │   │   ├── hashed.rs
│   │   │       │   │   │   └── raw.rs
│   │   │       │   │   └── mod.rs
│   │   │       │   └── mod.rs
│   │   │       ├── service/           # domain services murni
│   │   │       │   └── authentication/
│   │   │       │       ├── mod.rs
│   │   │       │       └── strategy/
│   │   │       │           ├── mod.rs
│   │   │       │           ├── password_based.rs
│   │   │       │           └── oauth2.rs
│   │   │       ├── repository/        # trait port (interface) untuk persistensi
│   │   │       │   └── mod.rs
│   │   │       ├── event/             # domain events
│   │   │       │   ├── mod.rs
│   │   │       │   └── user_registered.rs
│   │   │       ├── error.rs
│   │   │       └── lib.rs             # hanya re-export yang diperlukan
│   │   ├── payment/                   # bounded context lain
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       └── ...
│   │   └── ...                        # bounded context lain
│   ├── application/                   # use case / application services
│   │   ├── user/
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       ├── command/           # CQRS command
│   │   │       │   └── register_user.rs
│   │   │       ├── query/             # CQRS query
│   │   │       │   └── get_user.rs
│   │   │       ├── dto/               # data transfer objects
│   │   │       │   └── mod.rs
│   │   │       ├── handler.rs
│   │   │       └── lib.rs
│   │   └── payment/
│   │       └── ...
│   ├── infrastructure/                # implementasi konkret port
│   │   ├── persistence/
│   │   │   ├── postgres/
│   │   │   │   ├── Cargo.toml
│   │   │   │   └── src/
│   │   │   │       ├── user_repo.rs
│   │   │   │       ├── migration/
│   │   │   │       └── lib.rs
│   │   │   └── redis/
│   │   │       └── ...
│   │   ├── messaging/
│   │   │   ├── kafka/
│   │   │   └── ...
│   │   └── telemetry/
│   │       └── ...
│   └── interfaces/                    # adapter untuk delivery (HTTP, gRPC, CLI)
│       ├── rest/
│       │   ├── Cargo.toml
│       │   └── src/
│       │       ├── router.rs
│       │       └── handler/
│       └── cli/
│           └── ...
├── binaries/
│   ├── api-server/
│   │   ├── Cargo.toml
│   │   └── src/main.rs
│   └── worker/
│       └── ...
└── tests/                             # integration & e2e tests
    ├── integration/
    │   └── user_registration_test.rs
    └── e2e/
        └── full_flow_test.rs
```

Kedalaman folder bisa lebih dari 10 level. Tidak masalah selama setiap folder mencerminkan sebuah konsep yang jelas.

---

## 3. Konvensi Penamaan dan Aturan Gaya

### Penamaan

- **Nama file/modul**: snake_case, sama dengan nama modul. File lintas-modul induk selalu mod.rs.
- **Nama tipe**: UpperCamelCase; trait dengan akhiran …Ext, …Api hanya jika sangat diperlukan; lebih baik kata sifat jika memungkinkan (Authenticate bukan Authenticatable).
- **Fungsi**: snake_case; nama panjang deskriptif tidak masalah.
- **Konstanta**: SCREAMING_SNAKE_CASE untuk konstanta publik signifikan.
- Hindari singkatan yang tidak umum; eksplisit lebih baik.
- Makro: hanya digunakan jika diperlukan (misal derive), dan selalu didokumentasikan.

### Format Wajib (.rustfmt.toml)

```toml
max_width = 100
use_small_heuristics = "Max"
imports_granularity = "Crate"
group_imports = "StdExternalCrate"
```

### Clippy Ketat (clippy.toml)

```toml
cognitive-complexity-threshold = 25
```

Di lib.rs utama, tambahkan:

```rust
#![deny(clippy::all, clippy::pedantic, clippy::nursery, missing_docs)]
#![allow(clippy::module_name_repetitions)] // jika diperlukan
```

---

## 4. Organisasi Kode dalam Crate

Setiap crate hanya mengekspos API publik yang minimal. Seluruh detail internal ditandai pub(crate) atau private.

Contoh lib.rs domain crate:

```rust
#![deny(missing_docs)]
//! Domain logic for user management.

mod model;
mod service;
mod repository;
mod event;
mod error;

pub use model::entity::User;
pub use model::value_object::Email;
pub use service::authentication::Authenticate;
pub use repository::UserRepository;          // trait port
pub use error::UserDomainError;
pub use event::UserRegistered;
```

Setiap file minimal berisi:

- Dokumentasi modul dengan //! menjelaskan tanggung jawab.
- Semua fungsi publik harus didokumentasikan /// dengan contoh dan panic (jika ada, tapi di library tidak boleh panik).
- Gunakan atribut #[must_use] untuk fungsi yang hasilnya tidak boleh diabaikan.
- impl Trait hanya di return type jika kompleksitas internal tidak perlu diekspos; lebih baik definisikan newtype yang jelas.

---

## 5. Pola Data dan Manajemen State

### 5.1 Konfigurasi

Gunakan crate config dengan lapisan:
1. Default dalam kode.
2. File konfigurasi YAML/TOML per environment.
3. Environment variable override.
4. Secret dari vault/injector (tidak pernah hardcode).

### 5.2 Domain Model

- Entity memiliki ID kuat (newtype dari uuid::Uuid).
- Value object immutable, dijamin valid saat konstruksi.
- Domain service murni tanpa side effect.
- Semua perubahan state dikembalikan sebagai nilai baru (state immutable).

### 5.3 Komunikasi Antar Context

Gunakan domain event + message broker untuk eventual consistency lintas bounded context.

---

## 6. Penanganan Error & Logging

### 6.1 Error Library

Gunakan thiserror untuk mendefinisikan error enum per crate:

```rust
#[derive(Error, Debug)]
pub enum UserDomainError {
    #[error("Invalid email format: {0}")]
    InvalidEmail(String),
    #[error("User not found: {0}")]
    NotFound(UserId),
    #[error("Persistence failure: {0}")]
    Persistence(#[from] DbError),
}
```

Di library, tidak boleh menggunakan anyhow. Hanya binary (main.rs) yang boleh menangkap error generic dengan anyhow lalu dicatat.

### 6.2 Panic

Tidak ada unwrap di library code. Gunakan pattern match atau ?. Untuk invariant yang mustahil, gunakan debug_assert!; jika memang harus abort, gunakan unreachable! hanya setelah bukti formal.

### 6.3 Logging & Tracing

- Gunakan tracing crate dengan tracing-subscriber.
- Setiap fungsi publik me-log masuk/keluar dengan level DEBUG.
- Log level: ERROR untuk kegagalan sistem, WARN untuk degredasi, INFO untuk event bisnis penting, DEBUG sisanya.
- Harus ada span untuk setiap operasi bisnis.

---

## 7. Async dan Concurrency

- Runtime: tokio multi-thread, dengan feature flags eksplisit.
- Tidak memblokir async task. Semua operasi blocking harus dipindahkan ke spawn_blocking.
- Gunakan channel untuk komunikasi antar task.
- Trait Send + Sync dijamin oleh kompilator; hindari nested Rc/RefCell di kode async.

---

## 8. Pengujian (Testing)

- **Unit test**: di dalam file yang sama, modul #[cfg(test)]. Satu test per skenario, nama deskriptif.
- **Integration test**: di direktori tests/, menguji gabungan beberapa crate.
- **Property-based test**: gunakan proptest untuk value object.
- **Coverage**: minimal 90% untuk domain, 80% untuk application, diukur dengan cargo-tarpaulin.
- Test yang lambat: tag #[ignore] dan dijalankan di CI khusus.

---

## 9. Dokumentasi dan Komunikasi Tim

- Doc comment (///) WAJIB di setiap item publik, mencakup:
  - Tujuan.
  - Contoh pemakaian.
  - Error yang mungkin.
  - Jika fungsi panik (hanya di binary), tulis # Panics.
- Arsitektur: gunakan ADR di folder docs/adr/, format Markdown bernomor.
- Buku harian: setiap perubahan besar dimuat di CHANGELOG.md per crate.

---

## 10. Alat Bantu dan Otomasi (CI/CD)

CI Pipeline:
1. Format check (cargo fmt --check)
2. Lint (cargo clippy -- -D warnings)
3. Build (cargo build --release)
4. Test (cargo test --workspace --all-features)
5. Coverage report
6. Audit keamanan (cargo audit, cargo deny check)
7. Documentation build (cargo doc --no-deps --document-private-items)

Pre-commit hook: jalankan format dan clippy, dicegah jika gagal.

---

## WORKFLOW MANDATORY

Setiap perubahan wajib melalui:
1. `cargo build --workspace`
2. `cargo test --workspace`
3. `cargo clippy --workspace -- -D warnings`
4. `cargo fmt --all`
5. `cargo doc --workspace --no-deps --document-private-items`

---

## BUILD COMMANDS

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Single crate
cargo build -p xin-frontend

# Test specific
cargo test -p xin-typing typing::tests::test_refinement

# Benchmark
cargo bench -p xin-codegen

# LSP
cargo run -p xin-lsp

# Check all
cargo fmt --check && cargo clippy -- -D warnings && cargo test
```

---

## RUST EDITION

**Semua crate wajib pakai Rust Edition 2024**

---

## RULES

| DO | DO NOT |
|----|--------|
| ✓ Pakai Result<T, E> | ✗ Jangan unwrap/expect/panic di library |
| ✓ Tambah doc comment /// | ✗ Jangan lupa dokumentasi |
| ✓ Update workspace kalau tambah crate | ✗ Jangan violate naming conventions |
| ✓ 90%+ test coverage untuk domain | ✗ Jangan push tanpa test |
| ✓ Gunakan thiserror untuk error | ✗ Jangan pakai anyhow di library |

---

## 11. Code Metrics & File Density (MANDATORY)

### 11.1 Hard Limits

| Metric | Limit |
|--------|-------|
| **SLOC per file** | Max 200 lines (no exceptions) |
| **Documentation ratio** | Min 20% of file must be docs/comments |
| **Files per folder** | Max 5 files (exceed → refactor to subdirectory) |
| **Folder depth** | Unlimited (10+ levels allowed) |

### 11.2 Enforcement

- **When SLOC > 200**: Split module into new subdirectory with its own `mod.rs`.
- **When files > 5**: Create new subdirectory to group by responsibility.
- **Documentation**: Every public API must have `///` doc comment; modules have `//!`.
- Use `//` for explaining internal logic within functions.

### 11.3 Examples

```
xinc/src/lexer/token/literal/string/escape/validator/unicode/normalization/scalar/mod.rs
└── Deep nesting is encouraged to maintain low file density.
```

---

## OODA LOOP PROTOCOL (For All Changes)

1. **OBSERVE**: Map tree, check cwd, scan `.md` docs for specs.
2. **ORIENT**: Verify SLOC and FILE_DENSITY of target module.
3. **DECIDE**: Plan atomic splits if metrics exceed limits.
4. **ACT**: Execute tool calls with **Verification Header**.

### Verification Header

Every action sequence must start with:

```
[PATH]: <sub_project/mod> | [STATS]: <sloc_count>/200 | [DENSITY]: <file_count>/5 | [DOCS]: 20%_MIN
```

---

## STATUS

Early development - Struktur workspace dasar telah dibangun dengan standar Clean Architecture.

---

*Standar ini membutuhkan investasi awal yang besar, namun hasilnya adalah sistem yang scalable, mudah dipahami, serta tahan terhadap perubahan persyaratan selama bertahun-tahun.*
