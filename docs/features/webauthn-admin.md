Here’s a clean `Feature.md` you can give to Claude Code. It explains your idea clearly without overcomplicating it.

---

# Feature: WebAuthn (YubiKey) Authentication for Admin Panel

## 🎯 Goal

Add strong multi-factor authentication (MFA) to the admin panel using **WebAuthn (YubiKey / FIDO2 security keys)**.

This feature is intended **only for admin accounts**, not regular users.

---

## 🧠 Context

Current system:

* Backend: Rust
* Auth: JWT-based authentication
* Deployment: Docker on VPS

Problem:

* Admin authentication relies only on password + JWT
* This is not secure enough for sensitive access

Solution:

* Add **WebAuthn as a second authentication factor**
* Require a hardware security key (e.g. YubiKey) for admin login

---

## 🔐 High-Level Flow

### Login Flow (Admin)

1. Admin submits username + password
2. Backend validates credentials
3. Backend returns a **temporary JWT** with `mfa_pending = true`
4. Frontend triggers WebAuthn authentication
5. Backend verifies WebAuthn response
6. Backend returns a **full JWT** with `mfa = true`

---

### Registration Flow (Admin)

1. Logged-in admin (already authenticated) goes to "Security Settings"
2. Clicks "Register Security Key"
3. Backend generates a WebAuthn registration challenge
4. Browser calls `navigator.credentials.create()`
5. YubiKey is used to create credentials
6. Backend verifies and stores:

   * Credential ID
   * Public key
   * Sign counter

---

## 🧩 Technical Requirements

### Backend (Rust)

Use a WebAuthn library (recommended):

* `webauthn-rs`

Responsibilities:

* Generate registration challenges
* Verify registration responses
* Generate authentication challenges
* Verify authentication responses

Store per admin user:

* `credential_id`
* `public_key`
* `sign_count`
* optional: multiple devices

---

### Frontend

Use WebAuthn browser APIs:

* Registration:

```js
navigator.credentials.create()
```

* Authentication:

```js
navigator.credentials.get()
```

UI requirements:

* "Register Security Key" button
* MFA step during login
* Error handling (key not present, timeout, etc.)

---

## 🌐 Infrastructure Requirements

* HTTPS is **mandatory** (WebAuthn will not work without it)
* Must use a **valid domain name** (not raw IP)
* Compatible with current Docker setup (no major changes required)

---

## ⚠️ Security Rules

* Do NOT issue full-access JWT before WebAuthn verification
* Temporary JWT must be limited (`mfa_pending = true`)
* Protect all admin routes with `mfa = true`
* Allow multiple security keys per admin (recommended)
* Provide recovery mechanism (backup codes or fallback)

---

## 🧱 Suggested JWT Structure

Temporary JWT:

```json
{
  "user_id": "...",
  "role": "admin",
  "mfa_pending": true
}
```

Full JWT:

```json
{
  "user_id": "...",
  "role": "admin",
  "mfa": true
}
```

---

## 🚀 Scope

### Included

* WebAuthn registration
* WebAuthn authentication
* Admin-only enforcement
* JWT integration

### Not Included (for now)

* Passwordless login
* User-facing WebAuthn
* Biometric-only auth

---

## 💡 Notes

* This feature significantly increases admin security
* WebAuthn is phishing-resistant (unlike TOTP)
* Slightly complex but worth implementing properly

---

## ✅ Expected Outcome

* Admin login requires:

  * Password
  * * YubiKey (or compatible security key)

* Unauthorized access becomes extremely difficult even if password is leaked

---

If you want, I can next generate:

* a **Rust backend skeleton (webauthn-rs)**
* or a **minimal frontend example for the WebAuthn calls**
