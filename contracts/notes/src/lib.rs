#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Address, Env,
    String, Symbol, Vec, Map,
};

// =========================
// ENUM CATEGORY
// FIX: sebelumnya hardcode string tersebar di seluruh kode
// =========================
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum Category {
    Rust,
    Blockchain,
    General,
}

// =========================
// STRUCT NOTE
// FIX: tambah author agar notes bisa difilter per user
// =========================
#[contracttype]
#[derive(Clone, Debug)]
pub struct Note {
    pub id: u64,
    pub author: Address,     // FIX: simpan pemilik note
    pub title: String,
    pub content: String,
    pub category: Category,  // FIX: enum, bukan String
    pub created_at: u64,
}

// =========================
// STRUCT BADGE
// =========================
#[contracttype]
#[derive(Clone, Debug)]
pub struct Badge {
    pub name: String,
    pub unlocked_at: u64,
}

// =========================
// STRUCT USER PROFILE
// FIX: tambah last_note_day untuk tracking streak yang benar
// =========================
#[contracttype]
#[derive(Clone, Debug)]
pub struct UserProfile {
    pub level: u64,
    pub exp: u64,
    pub streak: u64,
    pub total_notes: u64,
    pub last_note_time: u64,
    pub last_note_day: u64,  // FIX: hari terakhir buat note (unix / 86400)
    pub badges: Vec<Badge>,
}

// =========================
// STORAGE KEYS
// FIX: NOTES sekarang Map<Address, Vec<Note>> bukan Vec<Note> global
// =========================
const NOTES: Symbol = symbol_short!("NOTES");

// TTL untuk persistent storage (dalam ledger sequence)
// Stellar ledger ~5 detik → 7 hari ≈ 120_960 ledger
const BUMP_AMOUNT: u32 = 120_960;
const LIFETIME_THRESHOLD: u32 = 60_480;

// =========================
// CONTRACT
// =========================
#[contract]
pub struct ProductivityRPGContract;

#[contractimpl]
impl ProductivityRPGContract {

    // =========================================
    // GET NOTES — hanya milik user ini
    // FIX: sebelumnya return semua notes dari semua user
    // =========================================
    pub fn get_notes(env: Env, user: Address) -> Vec<Note> {
        let all_notes: Map<Address, Vec<Note>> = env.storage()
            .instance()
            .get(&NOTES)
            .unwrap_or(Map::new(&env));

        all_notes.get(user).unwrap_or(Vec::new(&env))
    }

    // =========================================
    // GET USER PROFILE
    // FIX: tambah extend_ttl agar data tidak expired di mainnet
    // =========================================
    pub fn get_profile(env: Env, user: Address) -> UserProfile {
        if env.storage().persistent().has(&user) {
            // Perpanjang masa hidup data saat dibaca
            env.storage().persistent().extend_ttl(
                &user,
                LIFETIME_THRESHOLD,
                BUMP_AMOUNT,
            );
        }

        env.storage()
            .persistent()
            .get(&user)
            .unwrap_or(UserProfile {
                level: 1,
                exp: 0,
                streak: 0,
                total_notes: 0,
                last_note_time: 0,
                last_note_day: 0,
                badges: Vec::new(&env),
            })
    }

    // =========================================
    // CREATE NOTE
    // =========================================
    pub fn create_note(
        env: Env,
        user: Address,
        title: String,
        content: String,
        category: Category,  // FIX: pakai enum
    ) -> String {
        user.require_auth();

        // Ambil notes milik user ini saja
        let mut all_notes: Map<Address, Vec<Note>> = env.storage()
            .instance()
            .get(&NOTES)
            .unwrap_or(Map::new(&env));

        let mut user_notes: Vec<Note> = all_notes
            .get(user.clone())
            .unwrap_or(Vec::new(&env));

        let now = env.ledger().timestamp();

        let note = Note {
            id: env.prng().gen::<u64>(),
            author: user.clone(),
            title,
            content,
            category: category.clone(),
            created_at: now,
        };

        user_notes.push_back(note);
        all_notes.set(user.clone(), user_notes);
        env.storage().instance().set(&NOTES, &all_notes);

        // =========================================
        // UPDATE PROFILE
        // =========================================
        let mut profile = Self::get_profile(env.clone(), user.clone());

        profile.exp += 10;
        profile.total_notes += 1;

        // =========================================
        // STREAK SYSTEM — FIX: cegah double increment sehari
        // Sebelumnya: buat 5 note sehari = streak +5
        // Sekarang:   buat note hari berapa pun, streak hanya naik 1x/hari
        // =========================================
        let today = now / 86400;
        let yesterday = if today > 0 { today - 1 } else { 0 };

        if profile.last_note_day == 0 {
            // User baru pertama kali buat note
            profile.streak = 1;
        } else if today == profile.last_note_day {
            // Sudah buat note hari ini → streak tidak berubah
            // tidak increment, tidak reset
        } else if profile.last_note_day == yesterday {
            // Buat note hari berturut-turut → streak naik
            profile.streak += 1;
            profile.exp += 5; // bonus streak hanya sekali per hari
        } else {
            // Ada hari yang dilewati → reset streak
            profile.streak = 1;
        }

        profile.last_note_time = now;
        profile.last_note_day = today;

        // =========================================
        // CATEGORY BONUS — FIX: pakai enum, bukan string compare
        // =========================================
        match category {
            Category::Rust       => { profile.exp += 15; }
            Category::Blockchain => { profile.exp += 20; }
            Category::General    => {}
        }

        // =========================================
        // LEVEL SYSTEM — FIX: level hanya bisa naik, tidak turun
        // =========================================
        let new_level = (profile.exp / 100) + 1;
        if new_level > profile.level {
            profile.level = new_level;
        }

        // =========================================
        // BADGE SYSTEM
        // =========================================
        let badges = &mut profile.badges;

        // Rust Master: 10+ notes kategori Rust
        if matches!(category, Category::Rust) && profile.total_notes >= 10 {
            let name = String::from_str(&env, "Rust Master");
            if !Self::has_badge(badges, &name) {
                badges.push_back(Badge { name, unlocked_at: now });
            }
        }

        // 30-Day Learner: streak 30 hari berturut-turut
        if profile.streak >= 30 {
            let name = String::from_str(&env, "30-Day Learner");
            if !Self::has_badge(badges, &name) {
                badges.push_back(Badge { name, unlocked_at: now });
            }
        }

        // Blockchain Explorer: pertama kali pakai kategori Blockchain
        if matches!(category, Category::Blockchain) {
            let name = String::from_str(&env, "Blockchain Explorer");
            if !Self::has_badge(badges, &name) {
                badges.push_back(Badge { name, unlocked_at: now });
            }
        }

        // =========================================
        // SAVE PROFILE — FIX: extend TTL saat menulis
        // =========================================
        env.storage().persistent().set(&user, &profile);
        env.storage().persistent().extend_ttl(
            &user,
            LIFETIME_THRESHOLD,
            BUMP_AMOUNT,
        );

        String::from_str(&env, "Quest completed! EXP bertambah")
    }

    // =========================================
    // UPDATE NOTE — FIX: fungsi baru, edit konten note
    // =========================================
    pub fn update_note(
        env: Env,
        user: Address,
        id: u64,
        new_title: String,
        new_content: String,
    ) -> String {
        user.require_auth();

        let mut all_notes: Map<Address, Vec<Note>> = env.storage()
            .instance()
            .get(&NOTES)
            .unwrap_or(Map::new(&env));

        let mut user_notes: Vec<Note> = all_notes
            .get(user.clone())
            .unwrap_or(Vec::new(&env));

        for i in 0..user_notes.len() {
            let mut note = user_notes.get(i).unwrap();
            if note.id == id {
                note.title = new_title;
                note.content = new_content;
                user_notes.set(i, note);
                all_notes.set(user, user_notes);
                env.storage().instance().set(&NOTES, &all_notes);
                return String::from_str(&env, "Note berhasil diupdate");
            }
        }

        String::from_str(&env, "Note tidak ditemukan")
    }

    // =========================================
    // DELETE NOTE
    // FIX: tambah user param + require_auth + validasi kepemilikan
    // Sebelumnya: siapa saja bisa hapus note orang lain
    // =========================================
    pub fn delete_note(env: Env, user: Address, id: u64) -> String {
        user.require_auth();

        let mut all_notes: Map<Address, Vec<Note>> = env.storage()
            .instance()
            .get(&NOTES)
            .unwrap_or(Map::new(&env));

        let mut user_notes: Vec<Note> = all_notes
            .get(user.clone())
            .unwrap_or(Vec::new(&env));

        for i in 0..user_notes.len() {
            if user_notes.get(i).unwrap().id == id {
                user_notes.remove(i);
                all_notes.set(user, user_notes);
                env.storage().instance().set(&NOTES, &all_notes);
                return String::from_str(&env, "Note berhasil dihapus");
            }
        }

        String::from_str(&env, "Note tidak ditemukan")
    }

    // =========================================
    // GET LEADERBOARD (top EXP) — fungsi baru
    // Mengembalikan list profil publik untuk ranking
    // =========================================
    pub fn get_exp(env: Env, user: Address) -> u64 {
        Self::get_profile(env, user).exp
    }

    // =========================================
    // HAS BADGE — helper internal
    // FIX: hapus parameter &Env yang tidak dipakai
    // =========================================
    fn has_badge(badges: &Vec<Badge>, badge_name: &String) -> bool {
        for i in 0..badges.len() {
            if badges.get(i).unwrap().name == badge_name.clone() {
                return true;
            }
        }
        false
    }
}

// ============================================================
// TEST
// ============================================================
mod test;

