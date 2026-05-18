# ProductivityRPG DApp

**ProductivityRPG DApp** - Gamified Decentralized Note-Taking System on Stellar Soroban

---

## Project Description

ProductivityRPG DApp is a decentralized smart contract solution built on the **Stellar blockchain** using the **Soroban SDK**. It transforms everyday note-taking into a gamified on-chain experience — users earn EXP, level up, maintain daily learning streaks, and unlock achievement badges, all stored transparently and immutably on the blockchain.

The system allows users to create, update, view, and delete personal learning notes. Each note is categorized (Rust, Blockchain, or General) and contributes to the user's RPG profile. All user profiles and note data are stored in per-wallet isolated contract storage, ensuring complete ownership and privacy without relying on any centralized database provider.

---

## Project Vision

Our vision is to redefine personal productivity by merging blockchain technology with gamification mechanics:

- **Decentralizing Data**: Moving note-taking from centralized servers to a global, distributed blockchain
- **Ensuring Ownership**: Every user's notes and RPG profile belong solely to their wallet address — no third party can read, edit, or delete them
- **Rewarding Consistency**: Incentivizing daily learning habits through EXP, streaks, and on-chain badge achievements
- **Guaranteeing Immutability**: Notes and progression data are stored permanently on-chain, tamper-proof by design
- **Building Trustless Systems**: Game logic, badge unlocking, and level progression are enforced by smart contract code — not by any company or server

We envision a future where personal productivity tools are sovereign, transparent, and truly owned by the people who use them.

---

## Key Features

### 1. **Gamified Note Creation**

- Create notes with a title, content, and learning category (Rust / Blockchain / General)
- Earn **+10 base EXP** per note created
- Receive **category mastery bonuses** — Rust (+15 EXP), Blockchain (+20 EXP)
- All EXP and progression updates happen atomically in the same transaction

### 2. **RPG Progression System**

- **Level System** — automatically level up every 100 EXP; level never decreases
- **Daily Streak** — write at least one note per day to maintain your streak and earn +5 bonus EXP
- **Streak Reset** — missing a day resets the streak back to 1, encouraging consistent habits
- **User Profile** — every wallet has a persistent on-chain profile tracking level, EXP, streak, total notes, and last activity timestamp

### 3. **On-Chain Badge System**

Unlock achievement badges stored permanently on the blockchain:

| Badge                  | Unlock Condition                                        |
|------------------------|---------------------------------------------------------|
| 🦀 Rust Master          | Write 10 or more notes in the `Rust` category           |
| ⛓️ Blockchain Explorer  | Write your first note in the `Blockchain` category      |
| 🔥 30-Day Learner       | Maintain a learning streak for 30 consecutive days      |

### 4. **Secure Note Management**

- **Update notes** — edit the title and content of any existing note you own
- **Delete notes** — permanently remove a note; only the original owner can delete their own notes
- **Per-user isolation** — notes are stored in a `Map<Address, Vec<Note>>` structure, ensuring no user can read or modify another user's data
- All write operations enforce `require_auth()` — every action must be cryptographically signed by the wallet owner

### 5. **Efficient Data Retrieval**

- Fetch all notes belonging to a specific user in a single contract call
- Retrieve full RPG profile (level, EXP, streak, badges) in a single call
- Query a user's total EXP for leaderboard integration
- Structured data representation for straightforward frontend integration

### 6. **Stellar Network Integration**

- Built using the modern **Soroban Smart Contract SDK v21**
- Leverages the high speed and low transaction cost of the Stellar network
- Uses `persistent()` storage with TTL extension to ensure long-term data durability on mainnet
- Scalable per-user architecture suitable for growing note collections

---

## Contract Details

| Property         | Value                                                              |
|------------------|--------------------------------------------------------------------|
| **Network**      | Stellar Testnet                                                    |
*contract id*               CAWWDJO4SD4XUXHUG23G7AC4HPRONIEHEFWY4W426ADYPEBULOUPMRHJ
| **Contract ID**  | ``        |
| **RPC Endpoint** | `https://soroban-testnet.stellar.org`                              |
| **Explorer**     | [stellar.expert/testnet](https://stellar.expert/explorer/testnet)  |
| **SDK Version**  | `soroban-sdk v21`                                                  |

> ⚠️ This contract is deployed on **Stellar Testnet** for development and demonstration purposes. Do not use real XLM.

---

## Contract Functions

### Write Functions *(require wallet signature)*

| Function       | Parameters                               | Description                              |
|----------------|------------------------------------------|------------------------------------------|
| `create_note`  | `user, title, content, category`         | Create a note, earn EXP, update profile  |
| `update_note`  | `user, id, new_title, new_content`       | Edit an existing note (owner only)       |
| `delete_note`  | `user, id`                               | Permanently delete a note (owner only)   |

### Read Functions *(public, no signature required)*

| Function       | Parameters | Description                                        |
|----------------|------------|----------------------------------------------------|
| `get_notes`    | `user`     | Get all notes belonging to a specific user         |
| `get_profile`  | `user`     | Get full RPG profile: level, EXP, streak, badges   |
| `get_exp`      | `user`     | Get total EXP of a user (for leaderboard display)  |

---

## Technical Requirements

- Rust programming language (stable)
- Soroban SDK v21
- Stellar blockchain network (Testnet / Mainnet)
- Soroban CLI

---

## Getting Started

### Prerequisites

Install Rust and the Soroban CLI:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Soroban CLI
cargo install --locked soroban-cli

# Add WASM target
rustup target add wasm32-unknown-unknown
```

Fund your testnet account using [Stellar Friendbot](https://friendbot.stellar.org).

### Build the Contract

```bash
soroban contract build
```

### Run Tests

```bash
cargo test
```

### Deploy to Testnet

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/productivity_rpg.wasm \
  --source YOUR_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015"
```

### Interact with the Contract

**Create a note:**
```bash
soroban contract invoke \
  --id \CAWWDJO4SD4XUXHUG23G7AC4HPRONIEHEFWY4W426ADYPEBULOUPMRHJ

  --source YOUR_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015" \
  -- create_note \
  --user YOUR_ADDRESS \
  --title "My First Soroban Note" \
  --content "Today I deployed a smart contract on Stellar." \
  --category '{"Rust": null}'
```

**Get all notes:**
```bash
soroban contract invoke \
  --id CAWWDJO4SD4XUXHUG23G7AC4HPRONIEHEFWY4W426ADYPEBULOUPMRHJ \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015" \
  -- get_notes \
  --user YOUR_ADDRESS
```

**Get user profile:**
```bash
soroban contract invoke \
  --id  CAWWDJO4SD4XUXHUG23G7AC4HPRONIEHEFWY4W426ADYPEBULOUPMRHJ\

  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015" \
  -- get_profile \
  --user YOUR_ADDRESS
```

---

## Project Structure

```
productivity-rpg/
├── src/
│   ├── lib.rs       # Smart contract logic (notes, RPG system, badges)
│   └── test.rs      # Unit and integration tests
├── Cargo.toml       # Dependencies and build configuration
└── README.md
```

---

## Future Scope

### Short-Term Enhancements

1. **Note Encryption** — End-to-end encryption of note content for enhanced privacy
2. **Additional Categories** — Expand beyond Rust and Blockchain to support any learning topic
3. **Rich Text Support** — Extend content storage to support Markdown formatting
4. **Search and Filter** — Query notes by category, date, or keyword

### Medium-Term Development

5. **Public Leaderboard** — On-chain ranking of top users by EXP across all wallets
6. **NFT Badges** — Mint earned badges as NFTs on the Stellar network for true digital ownership
7. **Note Sharing** — Allow users to mark specific notes as publicly readable
8. **XLM Reward Pool** — Distribute token rewards to users who maintain the longest streaks
9. **Inter-Contract Integration** — Allow other Soroban contracts to read and interact with note data

### Long-Term Vision

10. **Frontend dApp** — React-based web interface with Freighter wallet integration
11. **Decentralized UI Hosting** — Host the frontend on IPFS for fully decentralized access
12. **Cross-Chain Synchronization** — Extend note storage compatibility to multiple blockchain networks
13. **AI-Powered Summaries** — Optional integration to auto-summarize note collections
14. **DAO Governance** — Community-driven protocol upgrades and new badge proposals
15. **Decentralized Identity** — Integration with DID systems for portable user profiles

### Enterprise Features

16. **Team Workspaces** — Shared note boards with role-based access control
17. **Immutable Audit Logs** — Time-locked note records for compliance and documentation
18. **Corporate Knowledge Base** — Adapt the system for secure, on-chain team documentation

---

## Security

- All write operations enforce `require_auth()` — every action requires a valid cryptographic signature from the wallet owner
- Notes are stored in isolated per-user storage — no cross-user data access is possible
- Note deletion is restricted to the note's original owner
- Storage TTL is extended on every read and write to prevent data expiry on Stellar mainnet

---

**ProductivityRPG DApp** - Level Up Your Learning on the Blockchain