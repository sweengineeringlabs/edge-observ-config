#!/usr/bin/env bash
set -euo pipefail
SCM_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(dirname "$SCM_ROOT")"
echo "==> Installing git hooks"
git -C "$REPO_ROOT" config core.hooksPath scm/scripts/hooks
echo "==> Fetching dependencies"
(cd "$SCM_ROOT" && cargo fetch --locked)
echo "Bootstrap complete."
