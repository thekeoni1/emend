#!/usr/bin/env bash
# Writes the emend pre-push hook into this repository's hooks directory. The
# hook runs scripts/privacy.sh over the commits being pushed and blocks the
# push on any non-zero exit.
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

# A hooks path set in any git config would make git ignore .git/hooks, so
# the hook would never run.
if hooks_path=$(git config --get core.hooksPath); then
  echo "install-hooks: core.hooksPath is set to '$hooks_path'; git would not run .git/hooks/pre-push" >&2
  exit 1
fi

hook="$(git rev-parse --git-dir)/hooks/pre-push"
mkdir -p "$(dirname "$hook")"
cat >"$hook" <<'HOOK'
#!/usr/bin/env bash
# emend pre-push hook, written by scripts/install-hooks.sh. For each pushed
# ref it runs scripts/privacy.sh over <remote sha>..<local sha>, or over all
# of <local sha> when the remote does not have the ref yet. Deletions are
# skipped. Any non-zero exit from scripts/privacy.sh blocks the push.
set -uo pipefail

root=$(git rev-parse --show-toplevel) || exit 1
status=0
while read -r local_ref local_sha remote_ref remote_sha; do
  if [[ $local_sha =~ ^0+$ ]]; then
    echo "pre-push: skipping the deletion of $remote_ref"
    continue
  fi
  # privacy.sh checks commits. An annotated tag's own message is not
  # checked, so pushing anything but a commit is blocked.
  type=$(git cat-file -t "$local_sha" 2>/dev/null)
  if [ "$type" != commit ]; then
    echo "pre-push: $local_ref is not a commit (${type:-unreadable}); blocked"
    status=1
    continue
  fi
  if [[ $remote_sha =~ ^0+$ ]]; then
    range=$local_sha
  else
    range=$remote_sha..$local_sha
  fi
  echo "pre-push: scripts/privacy.sh --range $range ($local_ref)"
  if ! "$root/scripts/privacy.sh" --range "$range" </dev/null; then
    status=1
  fi
done
exit "$status"
HOOK
chmod 755 "$hook"
echo "install-hooks: wrote $hook"
