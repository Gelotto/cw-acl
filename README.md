# CosmWasm ACL

> Enterprise-grade Access Control List for the Cosmos ecosystem

## Overview

**CosmWasm ACL** is a composable smart contract that brings hierarchical, role-based access control to the Cosmos ecosystem. Instead of hardcoding authorization logic into every contract, delegate permission management to a dedicated, battle-tested ACL system.

### Why ACL Contracts Matter

Traditional smart contracts embed authorization logic directly in their code—a pattern that leads to:
- **Rigid permissions** that require contract upgrades to modify
- **Duplicated code** across contracts that need similar access patterns
- **Complex administration** when managing multiple contracts
- **No delegation** capabilities for operational control

CosmWasm ACL solves these problems by providing a queryable, on-chain permission system that any contract can integrate through simple queries.

## Key Features

### Hierarchical Path-Based Authorization
Grant access to resource paths like `/api/users` and automatically inherit access to child paths like `/api/users/123/profile`. Mirrors filesystem and REST API patterns developers already understand.

### Dual Authorization Model
- **Direct Principal Grants**: Assign specific permissions directly to addresses or identifiers
- **Role-Based Access Control (RBAC)**: Create reusable roles, grant them permissions, then assign roles to principals

### Time-Bound Permissions
Set TTL (time-to-live) on any permission grant. Perfect for temporary access, trial periods, or time-limited delegations.

### Composable Operator Model
ACL operators can be either:
- A simple **Cosmos address** for straightforward control
- **Another ACL contract** for multi-tier hierarchical authorization

### Query-Based Integration
Zero storage overhead in your contracts—just query the ACL to check permissions. No state synchronization required.

### Flexible Permission Checks
Query with `All` or `Any` test requirements to verify access to multiple resources in a single call.

## Architecture

### Authorization Model

CosmWasm ACL implements a dual-path authorization system:

```
┌─────────────┐
│  Principal  │ (wallet address, app ID, etc.)
└──────┬──────┘
       │
       ├─────────────────┐
       │                 │
       ▼                 ▼
┌─────────────┐   ┌──────────┐
│ Direct Path │   │   Role   │
│   Grant     │   │  Grant   │
└─────────────┘   └────-┬────┘
       │                │
       │                ▼
       │          ┌──────────┐
       │          │   Role   │
       │          │  Paths   │
       │          └─────┬────┘
       │                │
       └────────┬───────┘
                ▼
          ┌──────────┐
          │   Path   │
          │ /api/... │
          └──────────┘
```

### Hierarchical Path Resolution

Paths are evaluated hierarchically from most specific to least specific:

```
Request: /api/users/123/edit

Check order:
1. /api/users/123/edit  ← Most specific
2. /api/users/123
3. /api/users
4. /api                 ← Least specific (grants access to all children)
```

If a principal has access to any parent path, they inherit access to all child paths.

### Storage Model

The contract maintains five primary indexes:

- **`PRINCIPAL_PATH_AUTHORIZATIONS`**: Direct principal → path mappings
- **`PRINCIPAL_ROLE_AUTHORIZATIONS`**: Principal → role assignments
- **`ROLE_PATHS`**: Role → paths allowed for that role
- **`PATH_ROLES`**: Reverse index for efficient role lookup by path
- **`PATH_REF_COUNTS`**: Reference counting for path cleanup

## Usage Examples

### Basic Setup: Instantiate an ACL

```rust
use cw_acl::msg::InstantiateMsg;
use cw_acl::client::Operator;

let msg = InstantiateMsg {
    operator: Some(Operator::Address(admin_addr)),
    name: Some("MyApp ACL".to_string()),
    description: Some("Access control for MyApp resources".to_string()),
};
```

### Direct Permission Grant

Grant a user access to a specific resource path:

```rust
use cw_acl::msg::{ExecuteMsg, AllowMsg};

let msg = ExecuteMsg::Allow(AllowMsg {
    principal: "juno1abc...xyz".to_string(),
    path: "/api/contracts/withdraw".to_string(),
    ttl: None, // No expiration
});
```

### Time-Limited Access

Grant temporary access that expires after 30 days:

```rust
let msg = ExecuteMsg::Allow(AllowMsg {
    principal: "juno1def...uvw".to_string(),
    path: "/api/admin/settings".to_string(),
    ttl: Some(2592000), // 30 days in seconds
});
```

### Role-Based Authorization

Create a role and assign permissions:

```rust
use cw_acl::msg::{RoleExecuteMsg, CreateRoleMsg, AllowRoleMsg, GrantRoleMsg};

// 1. Create a role
let create_msg = ExecuteMsg::Role(RoleExecuteMsg::Create(CreateRoleMsg {
    name: "moderator".to_string(),
    description: Some("Can moderate content and ban users".to_string()),
    paths: Some(vec![
        "/api/content/moderate".to_string(),
        "/api/users/ban".to_string(),
    ]),
}));

// 2. Grant additional paths to the role later
let allow_msg = ExecuteMsg::Role(RoleExecuteMsg::Allow(AllowRoleMsg {
    role: "moderator".to_string(),
    path: "/api/reports/view".to_string(),
}));

// 3. Assign role to a principal
let grant_msg = ExecuteMsg::Role(RoleExecuteMsg::Grant(GrantRoleMsg {
    principal: "juno1mod...123".to_string(),
    role: "moderator".to_string(),
    ttl: None,
}));
```

### Hierarchical Access

Grant access to a parent path to automatically allow all child paths:

```rust
// Grant access to /api/users
let msg = ExecuteMsg::Allow(AllowMsg {
    principal: "juno1api...xyz".to_string(),
    path: "/api/users".to_string(),
    ttl: None,
});

// Principal now has access to:
// - /api/users
// - /api/users/123
// - /api/users/123/profile
// - /api/users/123/settings
// ... and any other child paths
```

### Checking Permissions from Another Contract

```rust
use cw_acl::msg::{QueryMsg, IsAllowedParams, TestRequirement};

// Query the ACL contract
let is_allowed: bool = deps.querier.query_wasm_smart(
    acl_contract_addr,
    &QueryMsg::IsAllowed(IsAllowedParams {
        principal: info.sender.to_string(),
        paths: vec!["/api/withdraw".to_string()],
        require: Some(TestRequirement::All),
        raise: Some(false), // Return bool instead of error
    }),
)?;

if !is_allowed {
    return Err(ContractError::Unauthorized {});
}
```

### Multi-Path Permission Check

Verify access to multiple resources with flexible requirements:

```rust
// Require ALL paths to be authorized
let all_allowed: bool = deps.querier.query_wasm_smart(
    acl_contract_addr,
    &QueryMsg::IsAllowed(IsAllowedParams {
        principal: "juno1usr...abc".to_string(),
        paths: vec![
            "/api/read".to_string(),
            "/api/write".to_string(),
        ],
        require: Some(TestRequirement::All),
        raise: Some(false),
    }),
)?;

// Require ANY path to be authorized (at least one)
let any_allowed: bool = deps.querier.query_wasm_smart(
    acl_contract_addr,
    &QueryMsg::IsAllowed(IsAllowedParams {
        principal: "juno1usr...abc".to_string(),
        paths: vec![
            "/api/admin".to_string(),
            "/api/moderator".to_string(),
        ],
        require: Some(TestRequirement::Any),
        raise: Some(false),
    }),
)?;
```

### Using Operator Helper in Your Contract

CosmWasm ACL provides a client helper to enforce operator checks:

```rust
use cw_acl::client::{ensure_is_allowed, Operator};

// In your contract's execute function
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let operator = OPERATOR.load(deps.storage)?;

    // Enforce that sender is authorized
    ensure_is_allowed(
        deps.querier,
        &info.sender,
        operator,
        || format!("/contracts/{}/execute", env.contract.address),
    )?;

    // ... rest of your logic
}
```

## Message Reference

### InstantiateMsg

Initialize a new ACL instance.

```rust
pub struct InstantiateMsg {
    pub operator: Option<Operator>,
    pub name: Option<String>,
    pub description: Option<String>,
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `operator` | `Operator` | No | Controls who can modify the ACL. Defaults to the instantiator address. |
| `name` | `String` | No | Human-readable ACL name (max 100 chars) |
| `description` | `String` | No | ACL description (max 1000 chars) |

#### Operator Types

```rust
pub enum Operator {
    Address(Addr),  // Single address controls the ACL
    Acl(Addr),      // Another ACL contract controls this one
}
```

### ExecuteMsg

All execute operations require operator authorization.

#### SetOperator

Transfer control of the ACL to a new operator.

```rust
ExecuteMsg::SetOperator(Operator)
```

#### Allow

Grant a principal direct access to a path.

```rust
ExecuteMsg::Allow(AllowMsg {
    principal: String,
    path: String,
    ttl: Option<u32>,
})
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `principal` | `String` | Yes | Address or identifier to grant access |
| `path` | `String` | Yes | Resource path (e.g., `/api/users`) |
| `ttl` | `u32` | No | Time-to-live in seconds |

#### Deny

Remove a principal's direct access to a path.

```rust
ExecuteMsg::Deny(DenyMsg {
    principal: String,
    path: String,
})
```

#### Role Operations

All role operations are under `ExecuteMsg::Role(RoleExecuteMsg)`:

**Create**: Initialize a new role
```rust
RoleExecuteMsg::Create(CreateRoleMsg {
    name: String,
    description: Option<String>,
    paths: Option<Vec<String>>,
})
```

**Allow**: Add a path to a role
```rust
RoleExecuteMsg::Allow(AllowRoleMsg {
    role: String,
    path: String,
})
```

**Deny**: Remove a path from a role
```rust
RoleExecuteMsg::Deny(DenyRoleMsg {
    role: String,
    path: String,
})
```

**Grant**: Assign a role to a principal
```rust
RoleExecuteMsg::Grant(GrantRoleMsg {
    principal: String,
    role: String,
    ttl: Option<u32>,
})
```

**Revoke**: Remove a role from a principal
```rust
RoleExecuteMsg::Revoke(RevokeRoleMsg {
    principal: String,
    role: String,
})
```

### QueryMsg

#### Acl

Get ACL metadata and configuration.

```rust
QueryMsg::Acl {}
```

**Response**:
```rust
pub struct AclResponse {
    pub operator: Operator,
    pub created_by: Addr,
    pub created_at: Timestamp,
    pub name: Option<String>,
    pub description: Option<String>,
    pub config: Config,
}
```

#### IsAllowed

Check if a principal has access to specified paths.

```rust
QueryMsg::IsAllowed(IsAllowedParams {
    principal: String,
    paths: Vec<String>,
    require: Option<TestRequirement>,
    raise: Option<bool>,
})
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `principal` | `String` | - | Address or ID to check |
| `paths` | `Vec<String>` | - | Resource paths to verify |
| `require` | `TestRequirement` | `All` | `All` or `Any` |
| `raise` | `bool` | `false` | If true, returns error instead of false |

**Response**: `bool` (unless `raise: true`, then errors on unauthorized)

#### Roles

List roles for a principal, or all roles if principal is None.

```rust
QueryMsg::Roles {
    principal: Option<String>
}
```

**Response**:
```rust
pub struct RolesResponse(pub Vec<RoleResponse>);

pub struct RoleResponse {
    pub name: String,
    pub description: Option<String>,
    pub created_at: Timestamp,
    pub created_by: Addr,
    pub n_principals: u32,
    pub expires_at: Option<Timestamp>,
}
```

#### Role

Get detailed information about a specific role.

```rust
QueryMsg::Role(String)
```

**Response**: `RoleResponse`

#### Paths

List paths authorized to a subject (ACL, role, or principal).

```rust
QueryMsg::Paths(PathsQueryParams {
    subject: Subject,
    limit: Option<u16>,
    start: Option<String>,
    stop: Option<String>,
    cursor: Option<String>,
})
```

**Subject**:
```rust
pub enum Subject {
    Acl,                   // All paths in the ACL
    Role(String),          // Paths assigned to a role
    Principal(String),     // Paths authorized to a principal
}
```

**Response**:
```rust
pub struct PathsResponse {
    pub cursor: Option<String>,
    pub paths: Vec<PathInfo>,
}

pub struct PathInfo {
    pub path: String,
    pub expires_at: Option<Timestamp>,
}
```

## Development

### Prerequisites

- Rust 1.65+
- `wasm32-unknown-unknown` target
- Docker (for optimized builds)

### Building

```bash
# Development build
cargo build

# Optimized WASM build
make build
```

The optimized build uses `cosmwasm/optimizer` and outputs to `./artifacts/`.

### Testing

```bash
# Run all tests
cargo test

# Run with backtrace
RUST_BACKTRACE=1 cargo test
```

### Generating Schema

```bash
cargo schema
```

Outputs JSON schemas to `./schema/` for integration and tooling.

### Local Deployment

For local development with Juno:

```bash
# Deploy to local network
make tag=<tag> deploy
```

Default tag is `dev`. See the [Juno local dev setup guide](https://docs.junonetwork.io/developer-guides/junod-local-dev-setup) for details on starting a local node.

## Integration Guide

### Adding ACL to Your Contract

**1. Add dependency**:
```toml
[dependencies]
cw-acl = { path = "../cw-acl" }
```

**2. Store ACL address in your contract state**:
```rust
use cw_storage_plus::Item;
use cosmwasm_std::Addr;

pub const ACL_ADDRESS: Item<Addr> = Item::new("acl_addr");
```

**3. Protect execute functions**:
```rust
use cw_acl::client::{ensure_is_allowed, Operator};

pub fn execute_sensitive_action(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    // Load ACL address
    let acl_addr = ACL_ADDRESS.load(deps.storage)?;

    // Check authorization
    ensure_is_allowed(
        deps.querier,
        &info.sender,
        Operator::Acl(acl_addr),
        || "/my-contract/sensitive-action".to_string(),
    )?;

    // Your protected logic here
    Ok(Response::new())
}
```

**4. Or use direct queries for complex checks**:
```rust
use cw_acl::msg::{QueryMsg, IsAllowedParams, TestRequirement};

let acl_addr = ACL_ADDRESS.load(deps.storage)?;

let is_allowed: bool = deps.querier.query_wasm_smart(
    acl_addr,
    &QueryMsg::IsAllowed(IsAllowedParams {
        principal: info.sender.to_string(),
        paths: vec!["/my-contract/admin".to_string()],
        require: Some(TestRequirement::All),
        raise: Some(false),
    }),
)?;
```

### Path Naming Conventions

Follow these best practices for path structure:

- **Use forward slashes**: `/api/resource/action`
- **Be hierarchical**: Organize from general to specific
- **Be consistent**: Use the same pattern across your contract
- **Include contract context**: `/mycontract/module/action`
- **Use kebab-case**: `/user-management/create-admin`

**Examples**:
- `/myapp/users/create`
- `/myapp/users/delete`
- `/myapp/content/publish`
- `/myapp/settings/modify`

## Best Practices

### Security Considerations

**1. Validate Path Formats**
Always use consistent, validated path formats. The contract normalizes paths but consistency helps prevent confusion.

**2. Use Roles for Groups, Direct Grants for Exceptions**
- Define roles for common permission sets (admin, moderator, user)
- Use direct grants for individual overrides or temporary access

**3. Set TTLs for Temporary Access**
Always use TTL for:
- Trial periods
- Temporary delegations
- Time-bound promotions

**4. Hierarchical Operator Chains**
You can chain ACL contracts:
```
Master ACL (Operator: Admin Address)
    ↓
Department ACL (Operator: Master ACL)
    ↓
Project ACL (Operator: Department ACL)
```

This allows delegation without losing top-level control.

**5. Regular Audits**
Periodically query and audit:
- Assigned roles (`Roles` query)
- Path permissions (`Paths` query)
- Expired but not revoked permissions

### Performance Considerations

**1. Batch Permission Checks**
When possible, check multiple paths in a single `IsAllowed` query rather than multiple queries.

**2. Cache ACL Address**
Store the ACL contract address in your contract state rather than passing it on every call.

**3. Use `raise: false` for Conditional Logic**
When you want to handle unauthorized cases gracefully, use `raise: false` to get a boolean instead of an error.

**4. Prefer Roles Over Many Direct Grants**
Roles are more efficient than managing hundreds of individual principal-path pairs.

### Path Organization Patterns

**Resource-Based**:
```
/users/create
/users/read
/users/update
/users/delete
```

**Action-Based**:
```
/create/user
/create/post
/create/comment
```

**Module-Based**:
```
/auth/login
/auth/register
/api/users
/api/posts
```

Choose a pattern and be consistent across your application.

## Use Cases

### DeFi Protocols

- **Multi-signature Operations**: Grant withdrawal rights to 3-of-5 multisig via roles
- **Tiered Access**: Admin role for protocol changes, operator role for parameter tuning
- **Time-locked Proposals**: Grant execution rights with TTL after timelock expires

### DAOs

- **Governance Hierarchies**: Council members, working groups, general members
- **Proposal Execution**: Grant execution rights to passed proposal contracts
- **Committee Permissions**: Marketing committee controls `/treasury/marketing`

### NFT Platforms

- **Minting Rights**: Artist role can mint under `/collections/[artist-name]`
- **Marketplace Operations**: Marketplace operators access `/marketplace/list`, `/marketplace/delist`
- **Royalty Management**: Collection owners manage `/collections/[id]/royalties`

### Gaming

- **Game Masters**: Administrative controls over game state
- **Player Tiers**: Different access levels (free, premium, VIP)
- **Seasonal Access**: Time-bound permissions for special events

### Enterprise Applications

- **Department Hierarchies**: Engineering, Sales, Marketing with separate permissions
- **Service Accounts**: Bot accounts with limited, auditable permissions
- **Compliance Trails**: All permission changes logged on-chain

## Frequently Asked Questions

**Q: Can I have multiple ACL contracts?**
A: Yes! You can instantiate multiple ACLs for different applications or modules.

**Q: What happens when a permission expires?**
A: The `IsAllowed` query will return `false` (or error if `raise: true`). You should manually revoke or re-grant to clean up storage.

**Q: Can a principal have both direct and role-based access to the same path?**
A: Yes. The authorization check succeeds if either direct or role-based access is valid.

**Q: How much does it cost to query the ACL?**
A: Queries are free (read-only). Execute messages (grants, revokes) cost gas like any other transaction.

**Q: Can I revoke the operator role?**
A: Yes, via `SetOperator`. Be careful—if you set an invalid operator, the ACL becomes immutable.

**Q: Are paths case-sensitive?**
A: Yes. `/API/Users` and `/api/users` are different paths.

## License

This project is licensed under the Apache License 2.0 - see the LICENSE file for details.

## Acknowledgments

Built with [CosmWasm](https://cosmwasm.com/), the secure smart contract platform for the Cosmos ecosystem.
