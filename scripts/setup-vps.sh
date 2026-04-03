#!/usr/bin/env bash
set -Eeuo pipefail
IFS=$'\n\t'

# Usage:
#   sudo ./scripts/setup-vps.sh [--admin-user <name>] [--admin-ssh-key "ssh-ed25519 AAAA..."] [--dry-run]
#
# This script is intentionally idempotent so it can be called by a Makefile.

ADMIN_USER="${VPS_ADMIN_USER:-}"
ADMIN_SSH_KEY="${VPS_ADMIN_SSH_KEY:-}"
DRY_RUN=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --admin-user)
      ADMIN_USER="$2"
      shift 2
      ;;
    --admin-ssh-key)
      ADMIN_SSH_KEY="$2"
      shift 2
      ;;
    --dry-run)
      DRY_RUN=1
      shift
      ;;
    *)
      echo "Unknown argument: $1" >&2
      exit 1
      ;;
  esac
done

log() {
  printf '[setup-vps] %s\n' "$*"
}

run() {
  if [[ "$DRY_RUN" -eq 1 ]]; then
    printf '[dry-run] %s\n' "$*"
    return 0
  fi
  eval "$@"
}

require_root() {
  if [[ "$EUID" -ne 0 ]]; then
    if command -v sudo >/dev/null 2>&1; then
      log "Re-running with sudo..."
      exec sudo -E bash "$0" "$@"
    fi
    echo "This script must run as root." >&2
    exit 1
  fi
}

setup_ufw() {
  log "Configuring UFW (22/80/443 only)..."
  run "ufw --force default deny incoming"
  run "ufw --force default allow outgoing"
  run "ufw --force allow OpenSSH"
  run "ufw --force allow 80/tcp"
  run "ufw --force allow 443/tcp"
  run "ufw --force enable"
}

setup_fail2ban() {
  log "Installing and enabling fail2ban..."
  run "apt-get install -y fail2ban"
  run "systemctl enable fail2ban"
  run "systemctl restart fail2ban"
}

setup_auto_updates() {
  log "Enabling unattended upgrades..."
  run "apt-get install -y unattended-upgrades apt-listchanges"
  run "cat > /etc/apt/apt.conf.d/20auto-upgrades <<'EOF'
APT::Periodic::Update-Package-Lists \"1\";
APT::Periodic::Unattended-Upgrade \"1\";
EOF"
  run "systemctl enable unattended-upgrades"
  run "systemctl restart unattended-upgrades"
}

install_docker() {
  if command -v docker >/dev/null 2>&1; then
    log "Docker already installed, ensuring Compose plugin is present..."
    run "apt-get install -y docker-compose-plugin"
    return
  fi

  log "Installing Docker Engine + Docker Compose plugin..."
  run "apt-get install -y ca-certificates curl gnupg"
  run "install -m 0755 -d /etc/apt/keyrings"
  run "curl -fsSL https://download.docker.com/linux/ubuntu/gpg | gpg --dearmor -o /etc/apt/keyrings/docker.gpg"
  run "chmod a+r /etc/apt/keyrings/docker.gpg"
  run "echo \"deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu $(. /etc/os-release && echo $VERSION_CODENAME) stable\" > /etc/apt/sources.list.d/docker.list"
  run "apt-get update"
  run "apt-get install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin"
  run "systemctl enable docker"
  run "systemctl restart docker"
}

setup_admin_user() {
  if [[ -z "$ADMIN_USER" ]]; then
    log "No --admin-user provided. Skipping user creation (export VPS_ADMIN_USER to enable)."
    return
  fi

  if id "$ADMIN_USER" >/dev/null 2>&1; then
    log "User '$ADMIN_USER' already exists."
  else
    log "Creating non-root sudo user '$ADMIN_USER'..."
    run "useradd -m -s /bin/bash '$ADMIN_USER'"
  fi

  run "usermod -aG sudo '$ADMIN_USER'"
  run "usermod -aG docker '$ADMIN_USER'"

  if [[ -n "$ADMIN_SSH_KEY" ]]; then
    log "Installing SSH key for '$ADMIN_USER'..."
    run "install -d -m 700 -o '$ADMIN_USER' -g '$ADMIN_USER' '/home/$ADMIN_USER/.ssh'"
    run "touch '/home/$ADMIN_USER/.ssh/authorized_keys'"
    run "grep -qxF '$ADMIN_SSH_KEY' '/home/$ADMIN_USER/.ssh/authorized_keys' || echo '$ADMIN_SSH_KEY' >> '/home/$ADMIN_USER/.ssh/authorized_keys'"
    run "chown '$ADMIN_USER:$ADMIN_USER' '/home/$ADMIN_USER/.ssh/authorized_keys'"
    run "chmod 600 '/home/$ADMIN_USER/.ssh/authorized_keys'"
  fi
}

main() {
  require_root "$@"

  log "Refreshing apt index..."
  run "apt-get update"
  run "apt-get install -y ufw"

  setup_ufw
  setup_fail2ban
  setup_auto_updates
  install_docker
  setup_admin_user

  log "Setup complete."
  log "Next: configure secrets in .env and run docker compose up -d"
}

main "$@"
