# Notifications — rev0auth

Liste des événements pour lesquels recevoir une notification Telegram (en tant qu'admin).

---

## Compte & Authentification

| # | Événement | Déclencheur | Priorité |
|---|-----------|-------------|----------|
| 1 | **Nouvelle inscription** | Quelqu'un crée un compte via le portail | Haute |
| 2 | **Demande d'accès en attente** | Si le flow nécessite une approbation admin | Haute |
| 3 | **Brute-force détecté** | 5+ échecs de connexion sur un même compte | Haute |
| 4 | **Compte verrouillé** | Rate limiter déclenché sur un pseudo | Moyenne |
| 5 | **Changement de mot de passe** | Un membre change son mot de passe | Basse |
| 6 | **Suppression de compte** | Un membre supprime son compte | Moyenne |

---

## Mur communautaire

| # | Événement | Déclencheur | Priorité |
|---|-----------|-------------|----------|
| 7 | **Nouveau message sur le mur** | `POST /members/wall` réussi | Haute |
| 8 | **Message supprimé par un membre** | Un membre supprime son propre post | Basse |

---

## Messages privés (admin ↔ membres)

| # | Événement | Déclencheur | Priorité |
|---|-----------|-------------|----------|
| 9 | **Nouveau message reçu** | Un membre envoie un message à l'admin | Haute |
| 10 | **Thread non lu depuis 24h** | Thread ouvert sans réponse admin depuis 24h | Moyenne |

---

## Donations

| # | Événement | Déclencheur | Priorité |
|---|-----------|-------------|----------|
| 11 | **Nouvelle donation soumise** | Un membre soumet une donation | Haute |
| 12 | **Donation en attente de review** | Rappel si donation non traitée depuis 48h | Moyenne |

---

## Sécurité & Système

| # | Événement | Déclencheur | Priorité |
|---|-----------|-------------|----------|
| 13 | **Connexion admin** | Login réussi sur le dashboard `/japprends` | Haute |
| 14 | **Tentative de connexion admin échouée** | Mauvais mot de passe sur `/japprends/login` | Haute |
| 15 | **Token JWT invalide répété** | Plusieurs requêtes avec token forgé/expiré | Moyenne |
| 16 | **Erreur serveur critique** | Panic ou erreur 500 non gérée | Haute |

---

## Format des messages Telegram suggéré

```
🔔 [rev0auth] Nouvelle inscription
👤 Pseudo : alice_42
🕐 2026-05-24 14:32

---

⚠️ [rev0auth] Brute-force détecté
👤 Pseudo : bob
🔁 5 tentatives échouées
🌐 IP : 192.168.1.10
🕐 2026-05-24 14:45

---

💬 [rev0auth] Nouveau message sur le mur
👤 alice_42 : "Salut tout le monde !"
🕐 2026-05-24 15:01
```

---

## Implémentation prévue

- Module `notify.rs` dans `crates/web/src/`
- `TelegramNotifier` avec token + chat_id depuis l'env (`TELEGRAM_BOT_TOKEN`, `TELEGRAM_CHAT_ID`)
- Fire-and-forget via `tokio::spawn` — ne bloque jamais la réponse HTTP
- Graceful degradation : si Telegram est down, log l'erreur mais ne fait pas échouer la requête
