# Product Requirements Document
## Generic Inventory Management API

| Field | Value |
|---|---|
| Version | 1.0.0 |
| Status | Draft |
| Last Updated | June 2026 |

---

## Table of Contents

1. [Overview](#1-overview)
2. [Goals & Non-Goals](#2-goals--non-goals)
3. [Stakeholders](#3-stakeholders)
4. [Core Concepts & Terminology](#4-core-concepts--terminology)
5. [Data Models](#5-data-models)
6. [Functional Requirements](#6-functional-requirements)
7. [Non-Functional Requirements](#7-non-functional-requirements)
8. [Authentication & Authorization](#8-authentication--authorization)
9. [Multi-Tenancy Model](#9-multi-tenancy-model)
10. [Business Rules & Validation](#10-business-rules--validation)
11. [API Behavior Standards](#11-api-behavior-standards)
12. [Error Handling Requirements](#12-error-handling-requirements)
13. [Audit & Activity Logging](#13-audit--activity-logging)
14. [Future Considerations](#14-future-considerations)

---

## 1. Overview

### 1.1 Problem Statement

Vendors across industries — retail, manufacturing, logistics, food & beverage, pharmaceuticals, e-commerce, and more — need to track and manage their inventory. Most existing solutions are tightly coupled to a specific business domain or vendor type, making them unusable for businesses with different operational models.

This PRD describes a **generic, vendor-agnostic REST API** that provides full CRUD (Create, Read, Update, Delete) operations for inventory management. Any vendor — regardless of their industry, inventory type, or scale — should be able to adopt this API without the system imposing domain-specific assumptions on them.

### 1.2 Product Vision

A single multi-tenant API platform that allows any vendor to manage their inventory programmatically. The API makes no assumptions about what an "item" is — a product, a raw material, a digital asset, a service bundle, or anything else is equally valid.

### 1.3 Scope

This document covers the API layer only. It does not cover any frontend UI, reporting dashboards, payment systems, or order management. It defines what the API must do, what constraints it must enforce, and what behaviors consumers can rely on.

---

## 2. Goals & Non-Goals

### 2.1 Goals

- Provide a complete CRUD API for inventory items that works for any vendor type.
- Support multi-tenancy so multiple vendors can use the same API instance in complete isolation from each other.
- Allow flexible, schema-light item definitions so vendors are not forced into rigid product structures.
- Provide reliable stock quantity tracking with support for reservations and adjustments.
- Support item organization through categories and tagging.
- Expose filtering, searching, sorting, and pagination on all list operations.
- Maintain a full audit trail of all inventory changes.
- Be stateless, horizontally scalable, and deployable by any integrating party.
- Follow widely accepted REST conventions so any HTTP client can consume the API.

### 2.2 Non-Goals

- This API does **not** handle order management or fulfillment workflows.
- This API does **not** process payments or integrate with payment gateways.
- This API does **not** manage supplier relationships or purchase orders.
- This API does **not** provide a frontend UI or admin dashboard.
- This API does **not** handle file or image storage for items (references to external asset URLs are supported, but hosting is out of scope).
- This API does **not** enforce pricing logic, tax calculations, or discount rules.
- This API does **not** perform warehouse or physical location management beyond simple location labels.

---

## 3. Stakeholders

| Role | Description |
|---|---|
| **Vendor (API Consumer)** | Any business or individual integrating this API to manage their own inventory. |
| **Vendor Admin** | A user within a vendor account who has full control over that vendor's inventory data and settings. |
| **Vendor Operator** | A user within a vendor account with limited write permissions (e.g., can update stock but not delete items). |
| **Vendor Read-Only User** | A user within a vendor account who can only read inventory data. |
| **Platform Admin** | An operator of the API platform itself, with cross-tenant administrative access. |
| **System Integrations** | Automated services (warehouse systems, e-commerce platforms, ERP systems) that consume the API on behalf of a vendor. |

---

## 4. Core Concepts & Terminology

### Vendor (Tenant)
A vendor is the top-level organizational entity in the system. All data is namespaced under a vendor. Vendors are fully isolated from one another — a vendor cannot see or modify another vendor's data under any circumstance.

### Item
An item is the core unit of inventory. An item represents anything a vendor wishes to track. The system does not care whether an item is a physical product, a digital good, a service, a raw material, or an abstract unit. Items belong to exactly one vendor.

### SKU (Stock Keeping Unit)
A SKU is a vendor-defined unique identifier for an item within that vendor's namespace. It must be unique per vendor but does not need to be globally unique. The API accepts and stores the vendor's own SKU without transformation.

### Variant
A variant is a child of an item that represents a specific configuration (e.g., size, color, weight). An item may have zero or more variants. Variants inherit attributes from their parent item but can override specific fields like price, SKU, or stock quantity.

### Category
A category is a vendor-defined grouping mechanism for items. Categories support a parent-child hierarchy (a tree structure). An item may belong to one or more categories.

### Tag
A tag is a free-form label that can be attached to an item for flexible grouping, search, and filtering beyond the category hierarchy.

### Stock Record
A stock record represents the current quantity state of an item (or item variant) at a given location. It tracks quantities including total on-hand, reserved, and available-to-promise.

### Stock Adjustment
A stock adjustment is a logged, reason-coded change to the on-hand quantity of a stock record. Every change to stock quantity must produce an adjustment record.

### Location
A location is a vendor-defined label representing a physical or logical place where stock is held (warehouse, shelf, bin, store, etc.). The API stores location labels as strings; it does not model complex warehouse topology.

### Attribute
An attribute is a key-value pair that can be attached to an item or variant. Attributes allow vendors to add domain-specific metadata without the API needing to know about those fields.

### Unit of Measure (UOM)
A UOM is a vendor-defined string describing how an item is counted or measured (e.g., "each", "kg", "liter", "box of 12"). The API stores this as a string and does not perform unit conversions.

---

## 5. Data Models

### 5.1 Vendor

| Field | Type | Required | Notes |
|---|---|---|---|
| `id` | UUID | System-generated | Primary identifier |
| `name` | String | Yes | Display name of the vendor |
| `slug` | String | System-generated | URL-safe unique identifier derived from name |
| `status` | Enum | Yes | `active`, `suspended`, `pending` |
| `contact_email` | String | Yes | Primary contact email |
| `metadata` | Key-value map | No | Arbitrary key-value pairs for vendor-level configuration |
| `created_at` | Timestamp | System-generated | UTC |
| `updated_at` | Timestamp | System-generated | UTC |

### 5.2 Item

| Field | Type | Required | Notes |
|---|---|---|---|
| `id` | UUID | System-generated | Primary identifier |
| `vendor_id` | UUID | System-generated | Foreign key to Vendor |
| `sku` | String | Yes | Vendor-assigned unique code within the vendor namespace |
| `name` | String | Yes | Human-readable item name |
| `description` | String | No | Long-form description |
| `status` | Enum | Yes | `active`, `inactive`, `archived` |
| `unit_of_measure` | String | No | Vendor-defined UOM label |
| `base_price` | Decimal | No | Informational only; no pricing logic is enforced |
| `currency_code` | String | No | ISO 4217 currency code (e.g., "USD") |
| `category_ids` | Array of UUID | No | IDs of categories this item belongs to |
| `tags` | Array of String | No | Free-form labels |
| `attributes` | Key-value map | No | Arbitrary metadata fields |
| `image_urls` | Array of String | No | External URLs to item images |
| `has_variants` | Boolean | System-derived | True if the item has at least one variant |
| `created_at` | Timestamp | System-generated | UTC |
| `updated_at` | Timestamp | System-generated | UTC |

### 5.3 Item Variant

| Field | Type | Required | Notes |
|---|---|---|---|
| `id` | UUID | System-generated | Primary identifier |
| `item_id` | UUID | Yes | Parent item |
| `vendor_id` | UUID | System-generated | Denormalized from parent for efficient querying |
| `sku` | String | Yes | Must be unique within the vendor namespace |
| `name` | String | Yes | Variant-specific name (e.g., "Red – Large") |
| `status` | Enum | Yes | `active`, `inactive`, `archived` |
| `option_values` | Key-value map | Yes | Defines the variant dimensions (e.g., `{"color": "red", "size": "L"}`) |
| `base_price` | Decimal | No | Overrides parent item price if set |
| `attributes` | Key-value map | No | Overrides or extends parent attributes |
| `image_urls` | Array of String | No | Variant-specific images |
| `created_at` | Timestamp | System-generated | UTC |
| `updated_at` | Timestamp | System-generated | UTC |

### 5.4 Category

| Field | Type | Required | Notes |
|---|---|---|---|
| `id` | UUID | System-generated | Primary identifier |
| `vendor_id` | UUID | System-generated | Categories are vendor-scoped |
| `name` | String | Yes | Category display name |
| `slug` | String | System-generated | URL-safe identifier, unique within vendor |
| `parent_id` | UUID | No | Null for root categories |
| `description` | String | No | Optional description |
| `sort_order` | Integer | No | For UI ordering hints |
| `attributes` | Key-value map | No | Arbitrary metadata |
| `created_at` | Timestamp | System-generated | UTC |
| `updated_at` | Timestamp | System-generated | UTC |

### 5.5 Stock Record

| Field | Type | Required | Notes |
|---|---|---|---|
| `id` | UUID | System-generated | Primary identifier |
| `vendor_id` | UUID | System-generated | Vendor scope |
| `item_id` | UUID | Yes | References either an item or a variant (see note below) |
| `variant_id` | UUID | No | Null if tracking at item level; set if tracking at variant level |
| `location` | String | No | Vendor-defined location label |
| `quantity_on_hand` | Integer | Yes | Total physical units present |
| `quantity_reserved` | Integer | Yes | Units committed but not yet fulfilled |
| `quantity_available` | Integer | System-derived | `quantity_on_hand - quantity_reserved` |
| `reorder_point` | Integer | No | Quantity at which a reorder alert is triggered |
| `reorder_quantity` | Integer | No | Suggested reorder amount |
| `updated_at` | Timestamp | System-generated | UTC |

> **Note:** When an item has variants, stock is tracked at the variant level. When an item has no variants, stock is tracked at the item level directly. A stock record cannot simultaneously have a `variant_id` and be the sole stock record for a variant-bearing item.

### 5.6 Stock Adjustment

| Field | Type | Required | Notes |
|---|---|---|---|
| `id` | UUID | System-generated | Primary identifier |
| `vendor_id` | UUID | System-generated | Vendor scope |
| `stock_record_id` | UUID | Yes | The stock record being adjusted |
| `adjustment_type` | Enum | Yes | See Section 10.2 |
| `quantity_delta` | Integer | Yes | Positive for additions, negative for subtractions |
| `quantity_before` | Integer | System-recorded | Snapshot of on-hand before adjustment |
| `quantity_after` | Integer | System-recorded | Snapshot of on-hand after adjustment |
| `reason` | String | No | Human-readable reason for the adjustment |
| `reference_id` | String | No | Vendor-supplied reference (e.g., order number, shipment ID) |
| `performed_by` | UUID | System-recorded | User or API key that made the change |
| `created_at` | Timestamp | System-generated | UTC; adjustments are immutable |

### 5.7 API Key

| Field | Type | Required | Notes |
|---|---|---|---|
| `id` | UUID | System-generated | Primary identifier |
| `vendor_id` | UUID | Yes | Scoped to a vendor |
| `name` | String | Yes | Human-readable label |
| `key_prefix` | String | System-generated | First 8 characters of the key, for display |
| `key_hash` | String | System-generated | Hashed secret; raw key is only shown once at creation |
| `role` | Enum | Yes | `admin`, `operator`, `read_only` |
| `status` | Enum | Yes | `active`, `revoked` |
| `last_used_at` | Timestamp | No | Updated on each authenticated request |
| `expires_at` | Timestamp | No | Null means no expiry |
| `created_at` | Timestamp | System-generated | UTC |

---

## 6. Functional Requirements

### 6.1 Vendor Management

**FR-V-01** The system must allow creation of a new vendor account with a unique name and contact email.

**FR-V-02** The system must allow reading a vendor's own profile data.

**FR-V-03** The system must allow updating a vendor's mutable fields (name, contact email, metadata).

**FR-V-04** The system must support soft-suspension of a vendor account without deleting data.

**FR-V-05** The system must prevent a suspended vendor's API keys from authenticating.

**FR-V-06** Platform admins must be able to list and manage all vendors across the platform.

---

### 6.2 Item Management

**FR-I-01** The system must allow creating new items scoped to the authenticated vendor.

**FR-I-02** The system must enforce SKU uniqueness within a vendor's namespace. Two different vendors may use the same SKU without conflict.

**FR-I-03** The system must allow reading a single item by its system ID.

**FR-I-04** The system must allow reading a single item by its vendor-assigned SKU.

**FR-I-05** The system must allow listing all items for a vendor with support for filtering by status, category, tags, and any top-level scalar attribute.

**FR-I-06** The system must allow full updates to an item (replacing all mutable fields).

**FR-I-07** The system must allow partial updates to an item (modifying only the fields provided in the request).

**FR-I-08** The system must allow soft-deletion (archiving) of an item. Archived items must not appear in default list results but must remain accessible via direct lookup and via explicit filter.

**FR-I-09** The system must not allow hard-deletion of an item that has associated stock records with non-zero quantities.

**FR-I-10** The system must allow hard-deletion of an item that has never had any stock movements and has zero on-hand across all locations.

**FR-I-11** The system must support bulk creation of items (up to a configurable batch limit per request).

**FR-I-12** The system must support bulk status updates (e.g., archiving multiple items in one request).

---

### 6.3 Variant Management

**FR-VR-01** The system must allow adding one or more variants to an existing item.

**FR-VR-02** When a first variant is added to an item, the system must automatically set `has_variants = true` on the parent item.

**FR-VR-03** Variant SKUs must be unique across all item SKUs and variant SKUs within the vendor namespace.

**FR-VR-04** The system must allow reading all variants of a given item.

**FR-VR-05** The system must allow reading a single variant by its ID or SKU.

**FR-VR-06** The system must allow updating variant-level fields including option values, price, and attributes.

**FR-VR-07** The system must allow archiving a variant. An archived variant's stock record must also become inaccessible in default queries.

**FR-VR-08** The system must not allow deleting the last active variant of an item if the item itself is still active and has a stock record.

**FR-VR-09** Each variant within a single item must have a unique combination of `option_values`. The system must reject creation of a variant that duplicates an existing variant's option signature.

---

### 6.4 Category Management

**FR-C-01** The system must allow creating root-level categories and sub-categories for a vendor.

**FR-C-02** The category tree must support arbitrary depth. The system must enforce a maximum nesting depth of 10 levels to prevent performance issues.

**FR-C-03** The system must allow reading the full category tree for a vendor in a single request.

**FR-C-04** The system must allow reading a flat list of all categories with optional filtering by parent.

**FR-C-05** The system must allow updating a category's name, description, and sort order.

**FR-C-06** The system must allow moving a category to a different parent (re-parenting).

**FR-C-07** The system must not allow a category to be set as its own ancestor (circular reference prevention).

**FR-C-08** The system must allow deleting a category only if it has no child categories and no items currently assigned to it.

**FR-C-09** Category slugs must be unique within the vendor namespace. If a slug conflicts, the system must auto-suffix to ensure uniqueness.

---

### 6.5 Stock Management

**FR-S-01** The system must maintain one stock record per item-location combination (or variant-location combination for items with variants).

**FR-S-02** The system must automatically create a stock record with zero quantities when an item (or variant) is created, for a default location.

**FR-S-03** The system must allow adding stock records for additional locations.

**FR-S-04** The system must allow reading the stock record for a given item at a given location.

**FR-S-05** The system must allow reading all stock records for a given item across all locations.

**FR-S-06** The system must allow reading an aggregate stock summary for an item (sum of on-hand, reserved, and available across all locations).

**FR-S-07** The system must allow adding to or subtracting from `quantity_on_hand` via a stock adjustment operation. Direct writes to `quantity_on_hand` are not permitted.

**FR-S-08** The system must allow setting an absolute `quantity_on_hand` value via a "correction" adjustment type. This is intended for periodic physical count reconciliation.

**FR-S-09** The system must allow increasing `quantity_reserved` by a specified amount. The system must reject this if it would cause `quantity_available` to fall below zero.

**FR-S-10** The system must allow decreasing `quantity_reserved` by a specified amount.

**FR-S-11** The system must allow reading the full history of stock adjustments for a given stock record, item, or vendor, with time-range and type filters.

**FR-S-12** Stock adjustments must be immutable once created. No update or delete operation may be performed on a stock adjustment record.

**FR-S-13** The system must support bulk stock adjustment operations (adjusting multiple stock records in a single atomic request). If any single adjustment in a bulk request fails validation, the entire batch must be rejected.

**FR-S-14** The system must emit a low-stock signal (via a queryable status flag on the stock record) when `quantity_available` falls at or below the `reorder_point`. This is informational only — the API does not send notifications.

---

### 6.6 Search & Discovery

**FR-SD-01** The system must support full-text search across item name, description, SKU, and tags.

**FR-SD-02** The system must support filtering items by one or more categories (including items in child categories when a parent is selected).

**FR-SD-03** The system must support filtering items by one or more tags.

**FR-SD-04** The system must support filtering items by status.

**FR-SD-05** The system must support filtering items by stock availability (e.g., in-stock, out-of-stock, low-stock).

**FR-SD-06** The system must support sorting item lists by name, SKU, created date, updated date, and base price.

**FR-SD-07** All list endpoints must support cursor-based pagination. Offset-based pagination may also be supported as an option but cursor-based must be the default.

**FR-SD-08** The system must allow attribute-level filtering using simple equality operators on item attributes.

---

### 6.7 API Key Management

**FR-AK-01** The system must allow a vendor admin to create API keys for their vendor, assigning a role to each.

**FR-AK-02** The system must show the full secret key exactly once — immediately upon creation — and never again. Subsequent reads show only the key prefix.

**FR-AK-03** The system must allow revoking an API key. Revoked keys must be rejected immediately on the next use.

**FR-AK-04** The system must allow setting an expiry on an API key at creation time. Expired keys must be rejected.

**FR-AK-05** The system must update `last_used_at` on the key record with each authenticated request.

**FR-AK-06** A vendor admin must be able to list all API keys for their vendor (displaying prefix, role, status, and last-used date).

---

## 7. Non-Functional Requirements

### 7.1 Performance

**NFR-P-01** All read (GET) operations on single resources must respond in under 100ms at the 95th percentile under normal load.

**NFR-P-02** All list operations with standard filters and pagination must respond in under 300ms at the 95th percentile.

**NFR-P-03** Write operations (create, update, adjust) must respond in under 200ms at the 95th percentile.

**NFR-P-04** Bulk operations (batch create/adjust) must respond within 2 seconds for batches up to the maximum allowed batch size.

**NFR-P-05** The system must not degrade in single-tenant performance as the total number of tenants on the platform grows.

### 7.2 Scalability

**NFR-SC-01** The API must be stateless at the application layer, enabling horizontal scaling.

**NFR-SC-02** The system must support at least 1,000 vendors, each with up to 1,000,000 items, without architectural changes.

**NFR-SC-03** The system must support at least 500 concurrent requests across all tenants without degradation.

### 7.3 Reliability & Availability

**NFR-R-01** The API must target 99.9% uptime (excluding scheduled maintenance).

**NFR-R-02** Stock adjustments must be transactional — a partial write must never leave stock records in an inconsistent state.

**NFR-R-03** In the case of a system crash during a bulk stock adjustment, the system must guarantee that either all adjustments in the batch were committed or none were.

**NFR-R-04** All persistent state changes must be written to durable storage before returning a success response to the caller.

### 7.4 Security

**NFR-SE-01** All API traffic must occur over TLS. Plain HTTP connections must be refused.

**NFR-SE-02** API keys must never be logged in plaintext in application logs, access logs, or error reports.

**NFR-SE-03** All inputs must be validated and sanitized before processing. Malformed inputs must return a structured error, not a system exception.

**NFR-SE-04** SQL and injection attacks must be mitigated through parameterized queries at the data layer.

**NFR-SE-05** Rate limiting must be applied per API key. Limits must be configurable and communicated clearly in response headers.

**NFR-SE-06** The system must not expose any internal infrastructure details (stack traces, database errors, file paths) in API responses.

### 7.5 Observability

**NFR-O-01** Every request must be assigned a unique request ID, included in the response headers.

**NFR-O-02** The system must emit structured logs for all requests including: timestamp, request ID, vendor ID, HTTP method, resource path (without sensitive query parameters), response status, and duration.

**NFR-O-03** The system must expose a health check endpoint that returns current status without requiring authentication.

**NFR-O-04** Key operational metrics (request count, error rate, response time percentiles, stock adjustment throughput) must be emitted to a metrics system.

### 7.6 Maintainability

**NFR-M-01** The API must be versioned. The version must be communicated in the request (via URL path prefix or header — to be decided at implementation time, not in scope of this PRD).

**NFR-M-02** Breaking changes to an existing API version are prohibited. Additive changes (new optional fields, new endpoints) are allowed.

**NFR-M-03** Deprecated fields must be flagged in responses before removal, with a minimum deprecation notice period of 3 months.

---

## 8. Authentication & Authorization

### 8.1 Authentication Mechanism

The API uses API key-based authentication. API keys are long-lived bearer tokens issued per vendor. Each key carries an embedded role claim that determines what operations it may perform. All requests must include the API key in the `Authorization` header using the Bearer scheme.

### 8.2 Role Definitions

| Role | Permissions Summary |
|---|---|
| `admin` | Full CRUD on all resources scoped to their vendor. Can manage API keys for their vendor. |
| `operator` | Can create, read, and update items, categories, variants, and stock records. Cannot delete items, manage API keys, or modify vendor settings. |
| `read_only` | Can read all resources scoped to their vendor. No write operations permitted. |
| `platform_admin` | Cross-tenant read access. Can suspend/activate vendors. Cannot read or modify inventory data belonging to a specific vendor unless that vendor's isolation is explicitly overridden by a platform-level operation. |

### 8.3 Authorization Rules

- Every authenticated request must be validated against the vendor scope embedded in the key. A key issued to Vendor A must never be able to access Vendor B's resources.
- Authorization failures must return the same response shape as authentication failures to avoid leaking whether a resource exists across tenants.
- Platform admin operations must be gated separately and must never be accessible to vendor-level keys, regardless of role.

### 8.4 Key Rotation

- Vendors may create a new key before revoking the old one to enable zero-downtime rotation.
- There is no limit on the number of active API keys per vendor.

---

## 9. Multi-Tenancy Model

### 9.1 Isolation Guarantee

Tenant isolation is a hard requirement. Data belonging to one vendor must never be visible to, modifiable by, or inferable by another vendor through any API operation, error message, or timing side channel.

### 9.2 Namespace Isolation

The following fields must be unique within a vendor namespace but are not globally unique:

- Item SKU
- Variant SKU
- Category slug

### 9.3 Resource Limits

Each vendor is subject to configurable resource limits. Default limits (adjustable per vendor by a platform admin):

| Resource | Default Limit |
|---|---|
| Items per vendor | 500,000 |
| Variants per item | 500 |
| Categories per vendor | 10,000 |
| Category nesting depth | 10 levels |
| API keys per vendor | 50 |
| Tags per item | 50 |
| Attributes per item | 100 |
| Items per bulk create request | 100 |
| Adjustments per bulk adjust request | 200 |

When a limit is reached, the system must return a structured error indicating which limit was hit. It must not silently truncate.

---

## 10. Business Rules & Validation

### 10.1 Item Rules

- An item's `sku` may not be changed after creation. If a vendor needs to reassign an SKU, they must archive the old item and create a new one.
- An item cannot be set to `active` status if all of its variants (if any exist) are archived.
- An item with `has_variants = true` cannot directly hold a stock record. Stock must be tracked at the variant level.
- Tags must not exceed 64 characters each. Attribute keys must not exceed 128 characters. Attribute values must not exceed 1,024 characters.

### 10.2 Stock Adjustment Types

| Type | Meaning | Effect on `quantity_on_hand` |
|---|---|---|
| `receipt` | Goods received into stock | Positive delta |
| `shipment` | Goods dispatched out of stock | Negative delta |
| `return` | Customer or supplier return received | Positive delta |
| `damage` | Items written off as damaged | Negative delta |
| `correction` | Physical count reconciliation | Sets to absolute value |
| `transfer_in` | Stock moved in from another location | Positive delta |
| `transfer_out` | Stock moved out to another location | Negative delta |
| `reservation` | Units reserved (committed) | No change to on-hand; increases `quantity_reserved` |
| `reservation_release` | Reserved units released | No change to on-hand; decreases `quantity_reserved` |

### 10.3 Stock Constraint Rules

- `quantity_on_hand` must never go below zero. Any adjustment that would cause a negative on-hand must be rejected with a descriptive error.
- `quantity_reserved` must never exceed `quantity_on_hand`. A reservation that would exceed on-hand must be rejected.
- `quantity_available` is always derived (`quantity_on_hand - quantity_reserved`) and must never be stored as a writable field.
- A correction adjustment must specify the target absolute quantity. It must not be expressed as a delta.
- Transfer operations (transfer_in / transfer_out) should reference the sibling stock record if both locations exist in the system, but this is advisory only — the API does not enforce cross-location double-entry.

### 10.4 Category Rules

- A category slug must be URL-safe (alphanumeric, hyphens only).
- Moving a category to a new parent must check for circular references before committing.
- Deleting a category must cascade to remove all item-to-category associations for that category, but must not affect the items themselves.

### 10.5 Variant Rules

- Two variants of the same item must not share an identical `option_values` map. Comparison is key-for-key and value-for-value.
- A variant's `option_values` must contain at least one key.

---

## 11. API Behavior Standards

### 11.1 Response Envelope

All responses must use a consistent JSON envelope structure.

**Successful single-resource response:**
```json
{
  "data": { ... },
  "meta": { "request_id": "..." }
}
```

**Successful list response:**
```json
{
  "data": [ ... ],
  "pagination": {
    "cursor_next": "...",
    "cursor_prev": "...",
    "has_next": true,
    "has_prev": false,
    "total_count": 1042
  },
  "meta": { "request_id": "..." }
}
```

**Error response:**
```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Human-readable message",
    "details": [ { "field": "sku", "issue": "SKU already in use" } ]
  },
  "meta": { "request_id": "..." }
}
```

### 11.2 Pagination

- Default page size: 20 records.
- Maximum page size: 200 records.
- Cursor tokens must be opaque to the client (base64-encoded or similar). Clients must not construct or parse cursor tokens.
- The `total_count` field in list responses represents the count of all matching records before pagination, not the count in the current page.
- When a filter changes, any previously-issued cursor becomes invalid and must return an error.

### 11.3 Sorting

- All list endpoints must accept a `sort_by` field and a `sort_order` (`asc` or `desc`).
- Default sort for items is `created_at desc`.
- Multi-field sort may be supported but is not required for v1.

### 11.4 Filtering

- Filter parameters must be passed as query string parameters.
- Multiple values for the same filter key must be treated as OR within that filter and AND across different filter keys.
- Example: `status=active&status=inactive` returns items that are active OR inactive. Adding `tag=electronics` further restricts to only items that are (active OR inactive) AND tagged "electronics".

### 11.5 Partial Updates

- Partial update operations must use the HTTP PATCH method.
- Only fields present in the request body must be modified. Fields absent from the body must remain unchanged.
- Explicit `null` values in the body must clear the field (if the field is nullable).

### 11.6 Idempotency

- Create operations may accept a vendor-supplied `idempotency_key` (a string UUID) in the request header.
- If a request with a given idempotency key has already succeeded within the last 24 hours, the system must return the original response without re-processing.
- If a request with a given idempotency key previously failed, a new attempt with the same key must retry the operation.

### 11.7 Soft Delete vs. Hard Delete

- All primary resources (items, variants, categories) support soft deletion via an `archived` status.
- Archived resources are excluded from default list results but remain accessible via direct lookup and via explicit `status=archived` filter.
- Hard deletion is only permitted when the resource has no dependent records or when all dependent records are also being deleted atomically.

### 11.8 Timestamps

- All timestamps must be in ISO 8601 format.
- All timestamps must be stored and returned in UTC.
- The system must not accept timestamps with timezone offsets other than UTC (Z).

### 11.9 Numeric Precision

- All decimal/monetary values must use decimal number types in JSON (not floating point). Clients should treat these as strings when precision matters.
- Quantities must be non-negative integers. Fractional quantities are not supported.

### 11.10 HTTP Status Codes

| Scenario | Code |
|---|---|
| Successful read | 200 OK |
| Successful creation | 201 Created |
| Successful update (with body) | 200 OK |
| Successful deletion | 204 No Content |
| Validation failure | 422 Unprocessable Entity |
| Authentication failure | 401 Unauthorized |
| Authorization failure | 403 Forbidden |
| Resource not found | 404 Not Found |
| Conflict (e.g., duplicate SKU) | 409 Conflict |
| Rate limit exceeded | 429 Too Many Requests |
| Server error | 500 Internal Server Error |
| Service unavailable | 503 Service Unavailable |

---

## 12. Error Handling Requirements

**EH-01** Every error response must include a machine-readable `code` string and a human-readable `message` string.

**EH-02** Validation errors must include a `details` array identifying the specific field(s) and the nature of each violation.

**EH-03** Not-found errors must not reveal whether the resource exists in another vendor's namespace. The response for "resource belongs to another vendor" must be identical to "resource does not exist."

**EH-04** Rate limit responses must include headers indicating the limit, remaining calls, and the time at which the window resets.

**EH-05** All 5xx errors must include the request ID so they can be correlated with server-side logs.

**EH-06** The API must never return an HTML error page. All errors, including unexpected server errors, must be returned as JSON in the standard error envelope.

**EH-07** Errors caused by downstream service failures must be distinguishable from errors caused by invalid client input through distinct error codes, but must not expose internal service names.

---

## 13. Audit & Activity Logging

**AL-01** Every state-changing operation (create, update, delete, stock adjustment) must produce an audit log entry.

**AL-02** Each audit entry must record: timestamp, vendor ID, the actor (user or API key ID), the operation type, the resource type, the resource ID, and a before/after snapshot of changed fields.

**AL-03** Audit logs must be immutable. No API or internal process may modify or delete an audit log entry.

**AL-04** Vendors must be able to query their own audit log via the API, with filtering by resource type, resource ID, actor, and time range.

**AL-05** Audit logs must be retained for a minimum of 12 months.

**AL-06** Audit logs for stock adjustments are provided through the Stock Adjustment history (Section 5.6) and the general audit log. Both views must be available.

---

## 14. Future Considerations

The following items are intentionally out of scope for v1 but should be kept in mind during system design to avoid architectural blockers.

**FC-01 — Webhook / Event Streaming:** Vendors will likely want push notifications when stock falls below reorder point, when items are updated externally, or when adjustments are committed. The system should be designed so that events can be emitted to a message bus without restructuring the core data layer.

**FC-02 — Import / Export:** Bulk CSV or JSON import and export of item catalogs and stock data is a common operational need. The data model should be flat enough to serialize cleanly.

**FC-03 — Reserved Stock Expiry:** Reservations placed but never fulfilled waste available stock. A future version may support automatic expiry of reservations after a configurable TTL.

**FC-04 — Multi-Currency:** The current model stores a single `base_price` with a `currency_code`. Future versions may need to support multiple price points per item per currency.

**FC-05 — Lot & Serial Number Tracking:** High-compliance industries (pharma, electronics) require tracking individual units by lot number or serial number. The current model does not support this but the attribute system could be extended for a lightweight implementation.

**FC-06 — Advanced Search:** The attribute key-value filtering in v1 is limited to equality. Future versions should consider supporting range queries, existence checks, and full-text search within attribute values.

**FC-07 — Quota Increases:** The per-vendor resource limits defined in Section 9.3 should be manageable via a self-serve or platform-admin interface in a future iteration.

**FC-08 — Composite Items / Bundles:** Some vendors need to model bundles (item A + item B = bundle C) where selling the bundle decrements stock from each component. This requires a bill-of-materials model that is out of scope for v1.

---

*End of Document*
