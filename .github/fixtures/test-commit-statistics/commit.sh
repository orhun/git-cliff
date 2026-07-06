#!/usr/bin/env bash
set -e

cat <<'EOF' > notes.txt
base
EOF
GIT_COMMITTER_DATE="2022-04-06 01:25:08" git add notes.txt
GIT_COMMITTER_DATE="2022-04-06 01:25:08" git commit -m "Initial commit"
git tag v0.1.0

cat <<'EOF' > notes.txt
base
alpha
beta
EOF
GIT_COMMITTER_DATE="2022-04-06 01:25:09" git add notes.txt
GIT_COMMITTER_DATE="2022-04-06 01:25:09" git commit -m "feat: expand notes"

cat <<'EOF' > notes.txt
base
beta
gamma
EOF
cat <<'EOF' > summary.txt
todo
done
EOF
GIT_COMMITTER_DATE="2022-04-06 01:25:10" git add notes.txt summary.txt
GIT_COMMITTER_DATE="2022-04-06 01:25:10" git commit -m "fix: refresh notes"
