set -e

GIT_COMMITTER_DATE="2025-06-24 21:01:21" git commit --allow-empty -m "init"
git tag v0.0.1

GIT_COMMITTER_DATE="2025-06-24 21:01:22" git commit --allow-empty -m "feat: add feature 0"
GIT_COMMITTER_DATE="2025-06-24 21:01:23" git commit --allow-empty -m "feat: add feature 1"
git tag v0.1.0
